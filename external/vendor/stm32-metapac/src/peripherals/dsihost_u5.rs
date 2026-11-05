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
    #[doc = "DSI Host version register."]
    #[inline(always)]
    pub const fn vr(self) -> crate::common::Reg<regs::Vr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DSI Host control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DSI Host clock control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DSI Host LTDC VCID register."]
    #[inline(always)]
    pub const fn lvcidr(self) -> crate::common::Reg<regs::Lvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DSI Host LTDC color coding register."]
    #[inline(always)]
    pub const fn lcolcr(self) -> crate::common::Reg<regs::Lcolcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DSI Host LTDC polarity configuration register."]
    #[inline(always)]
    pub const fn lpcr(self) -> crate::common::Reg<regs::Lpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DSI Host low-power mode configuration register."]
    #[inline(always)]
    pub const fn lpmcr(self) -> crate::common::Reg<regs::Lpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DSI Host protocol configuration register."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "DSI Host generic VCID register."]
    #[inline(always)]
    pub const fn gvcidr(self) -> crate::common::Reg<regs::Gvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "DSI Host mode configuration register."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "DSI Host video mode configuration register."]
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "DSI Host video packet configuration register."]
    #[inline(always)]
    pub const fn vpcr(self) -> crate::common::Reg<regs::Vpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "DSI Host video chunks configuration register."]
    #[inline(always)]
    pub const fn vccr(self) -> crate::common::Reg<regs::Vccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DSI Host video null packet configuration register."]
    #[inline(always)]
    pub const fn vnpcr(self) -> crate::common::Reg<regs::Vnpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DSI Host video HSA configuration register."]
    #[inline(always)]
    pub const fn vhsacr(self) -> crate::common::Reg<regs::Vhsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DSI Host video HBP configuration register."]
    #[inline(always)]
    pub const fn vhbpcr(self) -> crate::common::Reg<regs::Vhbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "DSI Host video line configuration register."]
    #[inline(always)]
    pub const fn vlcr(self) -> crate::common::Reg<regs::Vlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "DSI Host video VSA configuration register."]
    #[inline(always)]
    pub const fn vvsacr(self) -> crate::common::Reg<regs::Vvsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "DSI Host video VBP configuration register."]
    #[inline(always)]
    pub const fn vvbpcr(self) -> crate::common::Reg<regs::Vvbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "DSI Host video VFP configuration register."]
    #[inline(always)]
    pub const fn vvfpcr(self) -> crate::common::Reg<regs::Vvfpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "DSI Host video VA configuration register."]
    #[inline(always)]
    pub const fn vvacr(self) -> crate::common::Reg<regs::Vvacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "DSI Host LTDC command configuration register."]
    #[inline(always)]
    pub const fn lccr(self) -> crate::common::Reg<regs::Lccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "DSI Host command mode configuration register."]
    #[inline(always)]
    pub const fn cmcr(self) -> crate::common::Reg<regs::Cmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DSI Host generic header configuration register."]
    #[inline(always)]
    pub const fn ghcr(self) -> crate::common::Reg<regs::Ghcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "DSI Host generic payload data register."]
    #[inline(always)]
    pub const fn gpdr(self) -> crate::common::Reg<regs::Gpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "DSI Host generic packet status register."]
    #[inline(always)]
    pub const fn gpsr(self) -> crate::common::Reg<regs::Gpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 0."]
    #[inline(always)]
    pub const fn tccr0(self) -> crate::common::Reg<regs::Tccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 1."]
    #[inline(always)]
    pub const fn tccr1(self) -> crate::common::Reg<regs::Tccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 2."]
    #[inline(always)]
    pub const fn tccr2(self) -> crate::common::Reg<regs::Tccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 3."]
    #[inline(always)]
    pub const fn tccr3(self) -> crate::common::Reg<regs::Tccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 4."]
    #[inline(always)]
    pub const fn tccr4(self) -> crate::common::Reg<regs::Tccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 5."]
    #[inline(always)]
    pub const fn tccr5(self) -> crate::common::Reg<regs::Tccr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "DSI Host clock lane configuration register."]
    #[inline(always)]
    pub const fn clcr(self) -> crate::common::Reg<regs::Clcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "DSI Host clock lane timer configuration register."]
    #[inline(always)]
    pub const fn cltcr(self) -> crate::common::Reg<regs::Cltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "DSI Host data lane timer configuration register."]
    #[inline(always)]
    pub const fn dltcr(self) -> crate::common::Reg<regs::Dltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "DSI Host PHY control register."]
    #[inline(always)]
    pub const fn pctlr(self) -> crate::common::Reg<regs::Pctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "DSI Host PHY configuration register."]
    #[inline(always)]
    pub const fn pconfr(self) -> crate::common::Reg<regs::Pconfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "DSI Host PHY ULPS control register."]
    #[inline(always)]
    pub const fn pucr(self) -> crate::common::Reg<regs::Pucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "DSI Host PHY TX triggers configuration register."]
    #[inline(always)]
    pub const fn pttcr(self) -> crate::common::Reg<regs::Pttcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "DSI Host PHY status register."]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "DSI Host interrupt and status register 0."]
    #[inline(always)]
    pub const fn isr0(self) -> crate::common::Reg<regs::Isr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "DSI Host interrupt and status register 1."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "DSI Host interrupt enable register 0."]
    #[inline(always)]
    pub const fn ier0(self) -> crate::common::Reg<regs::Ier0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "DSI Host interrupt enable register 1."]
    #[inline(always)]
    pub const fn ier1(self) -> crate::common::Reg<regs::Ier1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "DSI Host force interrupt register 0."]
    #[inline(always)]
    pub const fn fir0(self) -> crate::common::Reg<regs::Fir0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "DSI Host force interrupt register 1."]
    #[inline(always)]
    pub const fn fir1(self) -> crate::common::Reg<regs::Fir1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "DSI Host data lane timer read configuration register."]
    #[inline(always)]
    pub const fn dltrcr(self) -> crate::common::Reg<regs::Dltrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "DSI Host video shadow control register."]
    #[inline(always)]
    pub const fn vscr(self) -> crate::common::Reg<regs::Vscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DSI Host LTDC current VCID register."]
    #[inline(always)]
    pub const fn lcvcidr(self) -> crate::common::Reg<regs::Lcvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "DSI Host LTDC current color coding register."]
    #[inline(always)]
    pub const fn lcccr(self) -> crate::common::Reg<regs::Lcccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "DSI Host low-power mode current configuration register."]
    #[inline(always)]
    pub const fn lpmccr(self) -> crate::common::Reg<regs::Lpmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "DSI Host video mode current configuration register."]
    #[inline(always)]
    pub const fn vmccr(self) -> crate::common::Reg<regs::Vmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "DSI Host video packet current configuration register."]
    #[inline(always)]
    pub const fn vpccr(self) -> crate::common::Reg<regs::Vpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "DSI Host video chunks current configuration register."]
    #[inline(always)]
    pub const fn vcccr(self) -> crate::common::Reg<regs::Vcccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "DSI Host video null packet current configuration register."]
    #[inline(always)]
    pub const fn vnpccr(self) -> crate::common::Reg<regs::Vnpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "DSI Host video HSA current configuration register."]
    #[inline(always)]
    pub const fn vhsaccr(self) -> crate::common::Reg<regs::Vhsaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "DSI Host video HBP current configuration register."]
    #[inline(always)]
    pub const fn vhbpccr(self) -> crate::common::Reg<regs::Vhbpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "DSI Host video line current configuration register."]
    #[inline(always)]
    pub const fn vlccr(self) -> crate::common::Reg<regs::Vlccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "DSI Host video VSA current configuration register."]
    #[inline(always)]
    pub const fn vvsaccr(self) -> crate::common::Reg<regs::Vvsaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "DSI Host video VBP current configuration register."]
    #[inline(always)]
    pub const fn vvbpccr(self) -> crate::common::Reg<regs::Vvbpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "DSI Host video VFP current configuration register."]
    #[inline(always)]
    pub const fn vvfpccr(self) -> crate::common::Reg<regs::Vvfpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "DSI Host video VA current configuration register."]
    #[inline(always)]
    pub const fn vvaccr(self) -> crate::common::Reg<regs::Vvaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "DSI Host FIFO and buffer status register."]
    #[inline(always)]
    pub const fn fbsr(self) -> crate::common::Reg<regs::Fbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "DSI Wrapper configuration register."]
    #[inline(always)]
    pub const fn wcfgr(self) -> crate::common::Reg<regs::Wcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "DSI Wrapper control register."]
    #[inline(always)]
    pub const fn wcr(self) -> crate::common::Reg<regs::Wcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "DSI Wrapper interrupt enable register."]
    #[inline(always)]
    pub const fn wier(self) -> crate::common::Reg<regs::Wier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "DSI Wrapper interrupt and status register."]
    #[inline(always)]
    pub const fn wisr(self) -> crate::common::Reg<regs::Wisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "DSI Wrapper interrupt flag clear register."]
    #[inline(always)]
    pub const fn wifcr(self) -> crate::common::Reg<regs::Wifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "DSI Wrapper PHY configuration register 0."]
    #[inline(always)]
    pub const fn wpcr0(self) -> crate::common::Reg<regs::Wpcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "DSI Wrapper regulator and PLL control register."]
    #[inline(always)]
    pub const fn wrpcr(self) -> crate::common::Reg<regs::Wrpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[doc = "DSI bias configuration register."]
    #[inline(always)]
    pub const fn bcfgr(self) -> crate::common::Reg<regs::Bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "DSI D-PHY clock band control register."]
    #[inline(always)]
    pub const fn dpcbcr(self) -> crate::common::Reg<regs::Dpcbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c04usize) as _) }
    }
    #[doc = "DSI D-PHY clock skew rate control register."]
    #[inline(always)]
    pub const fn dpcsrcr(self) -> crate::common::Reg<regs::Dpcsrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c34usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 0 band control register."]
    #[inline(always)]
    pub const fn dpdl0bcr(self) -> crate::common::Reg<regs::Dpdl0bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c70usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 0 skew rate control register."]
    #[inline(always)]
    pub const fn dpdl0srcr(self) -> crate::common::Reg<regs::Dpdl0srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ca0usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 1 band control register."]
    #[inline(always)]
    pub const fn dpdl1bcr(self) -> crate::common::Reg<regs::Dpdl1bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d08usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 1 skew rate control register."]
    #[inline(always)]
    pub const fn dpdl1srcr(self) -> crate::common::Reg<regs::Dpdl1srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d38usize) as _) }
    }
}
pub mod regs {
    #[doc = "DSI bias configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcfgr(pub u32);
    impl Bcfgr {
        #[doc = "Power-up This bit powers-up the reference bias for the MIPI D-PHY."]
        #[inline(always)]
        pub const fn pwrup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Power-up This bit powers-up the reference bias for the MIPI D-PHY."]
        #[inline(always)]
        pub fn set_pwrup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Bcfgr {
        #[inline(always)]
        fn default() -> Bcfgr {
            Bcfgr(0)
        }
    }
    impl core::fmt::Debug for Bcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcfgr").field("pwrup", &self.pwrup()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bcfgr {{ pwrup: {=bool:?} }}", self.pwrup())
        }
    }
    #[doc = "DSI Host clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
        #[inline(always)]
        pub const fn txeckdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
        #[inline(always)]
        pub fn set_txeckdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
        #[inline(always)]
        pub const fn tockdiv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
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
    #[doc = "DSI Host clock lane configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clcr(pub u32);
    impl Clcr {
        #[doc = "D-PHY clock control This bit controls the D-PHY clock state:."]
        #[inline(always)]
        pub const fn dpcc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY clock control This bit controls the D-PHY clock state:."]
        #[inline(always)]
        pub fn set_dpcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
        #[inline(always)]
        pub const fn acr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
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
    #[doc = "DSI Host clock lane timer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cltcr(pub u32);
    impl Cltcr {
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY clock lane takes to go from lowâ\u{80}\u{91}power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn lp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY clock lane takes to go from lowâ\u{80}\u{91}power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_lp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY clock lane takes to go from highâ\u{80}\u{91}speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY clock lane takes to go from highâ\u{80}\u{91}speed to low-power transmission measured in lane byte clock cycles."]
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
    #[doc = "DSI Host command mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmcr(pub u32);
    impl Cmcr {
        #[doc = "Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:."]
        #[inline(always)]
        pub const fn teare(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:."]
        #[inline(always)]
        pub fn set_teare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge request enable This bit enables the acknowledge request after each packet transmission:."]
        #[inline(always)]
        pub const fn are(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge request enable This bit enables the acknowledge request after each packet transmission:."]
        #[inline(always)]
        pub fn set_are(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsw0tx(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsw0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsw1tx(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsw1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsw2tx(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsw2tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsr0tx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsr0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsr1tx(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsr1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsr2tx(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsr2tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Generic long write transmission This bit configures the generic long write packet command transmission type :."]
        #[inline(always)]
        pub const fn glwtx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Generic long write transmission This bit configures the generic long write packet command transmission type :."]
        #[inline(always)]
        pub fn set_glwtx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub const fn dsw0tx(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub fn set_dsw0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:."]
        #[inline(always)]
        pub const fn dsw1tx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:."]
        #[inline(always)]
        pub fn set_dsw1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub const fn dsr0tx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub fn set_dsr0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DCS long write transmission This bit configures the DCS long write packet command transmission type:."]
        #[inline(always)]
        pub const fn dlwtx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "DCS long write transmission This bit configures the DCS long write packet command transmission type:."]
        #[inline(always)]
        pub fn set_dlwtx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Maximum read packet size This bit configures the maximum read packet size command transmission type:."]
        #[inline(always)]
        pub const fn mrdps(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Maximum read packet size This bit configures the maximum read packet size command transmission type:."]
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
    #[doc = "DSI Host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable This bit configures the DSI Host in either power-up mode or to reset."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable This bit configures the DSI Host in either power-up mode or to reset."]
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
    #[doc = "DSI Host data lane timer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dltcr(pub u32);
    impl Dltcr {
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn lp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_lp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hs2lp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
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
                "Dltcr {{ lp2hs_time: {=u16:?}, hs2lp_time: {=u16:?} }}",
                self.lp2hs_time(),
                self.hs2lp_time()
            )
        }
    }
    #[doc = "DSI Host data lane timer read configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dltrcr(pub u32);
    impl Dltrcr {
        #[doc = "Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
        #[inline(always)]
        pub const fn mrd_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
        #[inline(always)]
        pub fn set_mrd_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Dltrcr {
        #[inline(always)]
        fn default() -> Dltrcr {
            Dltrcr(0)
        }
    }
    impl core::fmt::Debug for Dltrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dltrcr").field("mrd_time", &self.mrd_time()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dltrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dltrcr {{ mrd_time: {=u16:?} }}", self.mrd_time())
        }
    }
    #[doc = "DSI D-PHY clock band control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpcbcr(pub u32);
    impl Dpcbcr {
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
    }
    impl Default for Dpcbcr {
        #[inline(always)]
        fn default() -> Dpcbcr {
            Dpcbcr(0)
        }
    }
    impl core::fmt::Debug for Dpcbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpcbcr").field("bc", &self.bc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpcbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpcbcr {{ bc: {=u8:?} }}", self.bc())
        }
    }
    #[doc = "DSI D-PHY clock skew rate control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpcsrcr(pub u32);
    impl Dpcsrcr {
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dpcsrcr {
        #[inline(always)]
        fn default() -> Dpcsrcr {
            Dpcsrcr(0)
        }
    }
    impl core::fmt::Debug for Dpcsrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpcsrcr").field("src", &self.src()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpcsrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpcsrcr {{ src: {=u8:?} }}", self.src())
        }
    }
    #[doc = "DSI D-PHY data lane 0 band control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl0bcr(pub u32);
    impl Dpdl0bcr {
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Dpdl0bcr {
        #[inline(always)]
        fn default() -> Dpdl0bcr {
            Dpdl0bcr(0)
        }
    }
    impl core::fmt::Debug for Dpdl0bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpdl0bcr").field("bc", &self.bc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpdl0bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpdl0bcr {{ bc: {=u8:?} }}", self.bc())
        }
    }
    #[doc = "DSI D-PHY data lane 0 skew rate control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl0srcr(pub u32);
    impl Dpdl0srcr {
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dpdl0srcr {
        #[inline(always)]
        fn default() -> Dpdl0srcr {
            Dpdl0srcr(0)
        }
    }
    impl core::fmt::Debug for Dpdl0srcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpdl0srcr").field("src", &self.src()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpdl0srcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpdl0srcr {{ src: {=u8:?} }}", self.src())
        }
    }
    #[doc = "DSI D-PHY data lane 1 band control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl1bcr(pub u32);
    impl Dpdl1bcr {
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Dpdl1bcr {
        #[inline(always)]
        fn default() -> Dpdl1bcr {
            Dpdl1bcr(0)
        }
    }
    impl core::fmt::Debug for Dpdl1bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpdl1bcr").field("bc", &self.bc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpdl1bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpdl1bcr {{ bc: {=u8:?} }}", self.bc())
        }
    }
    #[doc = "DSI D-PHY data lane 1 skew rate control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl1srcr(pub u32);
    impl Dpdl1srcr {
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dpdl1srcr {
        #[inline(always)]
        fn default() -> Dpdl1srcr {
            Dpdl1srcr(0)
        }
    }
    impl core::fmt::Debug for Dpdl1srcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpdl1srcr").field("src", &self.src()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpdl1srcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpdl1srcr {{ src: {=u8:?} }}", self.src())
        }
    }
    #[doc = "DSI Host FIFO and buffer status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fbsr(pub u32);
    impl Fbsr {
        #[doc = "Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:."]
        #[inline(always)]
        pub const fn vcwfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:."]
        #[inline(always)]
        pub fn set_vcwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:."]
        #[inline(always)]
        pub const fn vcwff(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:."]
        #[inline(always)]
        pub fn set_vcwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub const fn vpwfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_vpwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub const fn vpwff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_vpwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub const fn acwfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub fn set_acwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub const fn acwff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub fn set_acwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub const fn apwfe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_apwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub const fn apwff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_apwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub const fn vpbe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_vpbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub const fn vpbf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_vpbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub const fn acbe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub fn set_acbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub const fn acbf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub fn set_acbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub const fn apbe(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_apbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub const fn apbf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_apbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Fbsr {
        #[inline(always)]
        fn default() -> Fbsr {
            Fbsr(0)
        }
    }
    impl core::fmt::Debug for Fbsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fbsr")
                .field("vcwfe", &self.vcwfe())
                .field("vcwff", &self.vcwff())
                .field("vpwfe", &self.vpwfe())
                .field("vpwff", &self.vpwff())
                .field("acwfe", &self.acwfe())
                .field("acwff", &self.acwff())
                .field("apwfe", &self.apwfe())
                .field("apwff", &self.apwff())
                .field("vpbe", &self.vpbe())
                .field("vpbf", &self.vpbf())
                .field("acbe", &self.acbe())
                .field("acbf", &self.acbf())
                .field("apbe", &self.apbe())
                .field("apbf", &self.apbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fbsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fbsr {{ vcwfe: {=bool:?}, vcwff: {=bool:?}, vpwfe: {=bool:?}, vpwff: {=bool:?}, acwfe: {=bool:?}, acwff: {=bool:?}, apwfe: {=bool:?}, apwff: {=bool:?}, vpbe: {=bool:?}, vpbf: {=bool:?}, acbe: {=bool:?}, acbf: {=bool:?}, apbe: {=bool:?}, apbf: {=bool:?} }}" , self . vcwfe () , self . vcwff () , self . vpwfe () , self . vpwff () , self . acwfe () , self . acwff () , self . apwfe () , self . apwff () , self . vpbe () , self . vpbf () , self . acbe () , self . acbf () , self . apbe () , self . apbf ())
        }
    }
    #[doc = "DSI Host force interrupt register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir0(pub u32);
    impl Fir0 {
        #[doc = "Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0."]
        #[inline(always)]
        pub const fn fae0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0."]
        #[inline(always)]
        pub fn set_fae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1."]
        #[inline(always)]
        pub const fn fae1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1."]
        #[inline(always)]
        pub fn set_fae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2."]
        #[inline(always)]
        pub const fn fae2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2."]
        #[inline(always)]
        pub fn set_fae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3."]
        #[inline(always)]
        pub const fn fae3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3."]
        #[inline(always)]
        pub fn set_fae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4."]
        #[inline(always)]
        pub const fn fae4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4."]
        #[inline(always)]
        pub fn set_fae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5."]
        #[inline(always)]
        pub const fn fae5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5."]
        #[inline(always)]
        pub fn set_fae5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6."]
        #[inline(always)]
        pub const fn fae6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6."]
        #[inline(always)]
        pub fn set_fae6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7."]
        #[inline(always)]
        pub const fn fae7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7."]
        #[inline(always)]
        pub fn set_fae7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8."]
        #[inline(always)]
        pub const fn fae8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8."]
        #[inline(always)]
        pub fn set_fae8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9."]
        #[inline(always)]
        pub const fn fae9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9."]
        #[inline(always)]
        pub fn set_fae9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10."]
        #[inline(always)]
        pub const fn fae10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10."]
        #[inline(always)]
        pub fn set_fae10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11."]
        #[inline(always)]
        pub const fn fae11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11."]
        #[inline(always)]
        pub fn set_fae11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12."]
        #[inline(always)]
        pub const fn fae12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12."]
        #[inline(always)]
        pub fn set_fae12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13."]
        #[inline(always)]
        pub const fn fae13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13."]
        #[inline(always)]
        pub fn set_fae13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14."]
        #[inline(always)]
        pub const fn fae14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14."]
        #[inline(always)]
        pub fn set_fae14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15."]
        #[inline(always)]
        pub const fn fae15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15."]
        #[inline(always)]
        pub fn set_fae15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Force PHY error 0 Writing one to this bit forces a PHY error 0."]
        #[inline(always)]
        pub const fn fpe0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 0 Writing one to this bit forces a PHY error 0."]
        #[inline(always)]
        pub fn set_fpe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Force PHY error 1 Writing one to this bit forces a PHY error 1."]
        #[inline(always)]
        pub const fn fpe1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 1 Writing one to this bit forces a PHY error 1."]
        #[inline(always)]
        pub fn set_fpe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Force PHY error 2 Writing one to this bit forces a PHY error 2."]
        #[inline(always)]
        pub const fn fpe2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 2 Writing one to this bit forces a PHY error 2."]
        #[inline(always)]
        pub fn set_fpe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Force PHY error 3 Writing one to this bit forces a PHY error 3."]
        #[inline(always)]
        pub const fn fpe3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 3 Writing one to this bit forces a PHY error 3."]
        #[inline(always)]
        pub fn set_fpe3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Force PHY error 4 Writing one to this bit forces a PHY error 4."]
        #[inline(always)]
        pub const fn fpe4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 4 Writing one to this bit forces a PHY error 4."]
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
    #[doc = "DSI Host force interrupt register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir1(pub u32);
    impl Fir1 {
        #[doc = "Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission."]
        #[inline(always)]
        pub const fn ftohstx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission."]
        #[inline(always)]
        pub fn set_ftohstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force timeout low-power reception Writing one to this bit forces a timeout low-power reception."]
        #[inline(always)]
        pub const fn ftolprx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force timeout low-power reception Writing one to this bit forces a timeout low-power reception."]
        #[inline(always)]
        pub fn set_ftolprx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force ECC single-bit error Writing one to this bit forces a ECC single-bit error."]
        #[inline(always)]
        pub const fn feccse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECC single-bit error Writing one to this bit forces a ECC single-bit error."]
        #[inline(always)]
        pub fn set_feccse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error."]
        #[inline(always)]
        pub const fn feccme(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error."]
        #[inline(always)]
        pub fn set_feccme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force CRC error Writing one to this bit forces a CRC error."]
        #[inline(always)]
        pub const fn fcrce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force CRC error Writing one to this bit forces a CRC error."]
        #[inline(always)]
        pub fn set_fcrce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force packet size error Writing one to this bit forces a packet size error."]
        #[inline(always)]
        pub const fn fpse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force packet size error Writing one to this bit forces a packet size error."]
        #[inline(always)]
        pub fn set_fpse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Force EoTp error Writing one to this bit forces a EoTp error."]
        #[inline(always)]
        pub const fn feotpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force EoTp error Writing one to this bit forces a EoTp error."]
        #[inline(always)]
        pub fn set_feotpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Force LTDC payload write error Writing one to this bit forces a LTDC payload write error."]
        #[inline(always)]
        pub const fn flpwre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Force LTDC payload write error Writing one to this bit forces a LTDC payload write error."]
        #[inline(always)]
        pub fn set_flpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Force generic command write error Writing one to this bit forces a generic command write error."]
        #[inline(always)]
        pub const fn fgcwre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic command write error Writing one to this bit forces a generic command write error."]
        #[inline(always)]
        pub fn set_fgcwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force generic payload write error Writing one to this bit forces a generic payload write error."]
        #[inline(always)]
        pub const fn fgpwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload write error Writing one to this bit forces a generic payload write error."]
        #[inline(always)]
        pub fn set_fgpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force generic payload transmit error Writing one to this bit forces a generic payload transmit error."]
        #[inline(always)]
        pub const fn fgptxe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload transmit error Writing one to this bit forces a generic payload transmit error."]
        #[inline(always)]
        pub fn set_fgptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Force generic payload read error Writing one to this bit forces a generic payload read error."]
        #[inline(always)]
        pub const fn fgprde(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload read error Writing one to this bit forces a generic payload read error."]
        #[inline(always)]
        pub fn set_fgprde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force generic payload receive error Writing one to this bit forces a generic payload receive error."]
        #[inline(always)]
        pub const fn fgprxe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload receive error Writing one to this bit forces a generic payload receive error."]
        #[inline(always)]
        pub fn set_fgprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force payload buffer underflow error Writing one to this bit forces a payload undrflow error."]
        #[inline(always)]
        pub const fn fpbue(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Force payload buffer underflow error Writing one to this bit forces a payload undrflow error."]
        #[inline(always)]
        pub fn set_fpbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("fpbue", &self.fpbue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fir1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fir1 {{ ftohstx: {=bool:?}, ftolprx: {=bool:?}, feccse: {=bool:?}, feccme: {=bool:?}, fcrce: {=bool:?}, fpse: {=bool:?}, feotpe: {=bool:?}, flpwre: {=bool:?}, fgcwre: {=bool:?}, fgpwre: {=bool:?}, fgptxe: {=bool:?}, fgprde: {=bool:?}, fgprxe: {=bool:?}, fpbue: {=bool:?} }}" , self . ftohstx () , self . ftolprx () , self . feccse () , self . feccme () , self . fcrce () , self . fpse () , self . feotpe () , self . flpwre () , self . fgcwre () , self . fgpwre () , self . fgptxe () , self . fgprde () , self . fgprxe () , self . fpbue ())
        }
    }
    #[doc = "DSI Host generic header configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ghcr(pub u32);
    impl Ghcr {
        #[doc = "Type This field configures the packet data type of the header packet."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Type This field configures the packet data type of the header packet."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Channel This field configures the virtual channel ID of the header packet."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Channel This field configures the virtual channel ID of the header packet."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
        #[inline(always)]
        pub const fn wclsb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
        #[inline(always)]
        pub fn set_wclsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
        #[inline(always)]
        pub const fn wcmsb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
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
    #[doc = "DSI Host generic payload data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpdr(pub u32);
    impl Gpdr {
        #[doc = "Payload byte 1 This field indicates the byte 1 of the packet payload."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 1 This field indicates the byte 1 of the packet payload."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Payload byte 2 This field indicates the byte 2 of the packet payload."]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 2 This field indicates the byte 2 of the packet payload."]
        #[inline(always)]
        pub fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Payload byte 3 This field indicates the byte 3 of the packet payload."]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 3 This field indicates the byte 3 of the packet payload."]
        #[inline(always)]
        pub fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Payload byte 4 This field indicates the byte 4 of the packet payload."]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 4 This field indicates the byte 4 of the packet payload."]
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
    #[doc = "DSI Host generic packet status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpsr(pub u32);
    impl Gpsr {
        #[doc = "Command FIFO empty This bit indicates the empty status of the generic command FIFO:."]
        #[inline(always)]
        pub const fn cmdfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command FIFO empty This bit indicates the empty status of the generic command FIFO:."]
        #[inline(always)]
        pub fn set_cmdfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command FIFO full This bit indicates the full status of the generic command FIFO:."]
        #[inline(always)]
        pub const fn cmdff(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command FIFO full This bit indicates the full status of the generic command FIFO:."]
        #[inline(always)]
        pub fn set_cmdff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:."]
        #[inline(always)]
        pub const fn pwrfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:."]
        #[inline(always)]
        pub fn set_pwrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:."]
        #[inline(always)]
        pub const fn pwrff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:."]
        #[inline(always)]
        pub fn set_pwrff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:."]
        #[inline(always)]
        pub const fn prdfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:."]
        #[inline(always)]
        pub fn set_prdfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:."]
        #[inline(always)]
        pub const fn prdff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:."]
        #[inline(always)]
        pub fn set_prdff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:."]
        #[inline(always)]
        pub const fn rcb(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:."]
        #[inline(always)]
        pub fn set_rcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub const fn cmdbe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Command buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub fn set_cmdbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Command buffer full This bit indicates the full status of the generic command internal buffer:."]
        #[inline(always)]
        pub const fn cmdbf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Command buffer full This bit indicates the full status of the generic command internal buffer:."]
        #[inline(always)]
        pub fn set_cmdbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub const fn pbe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub fn set_pbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Payload buffer full This bit indicates the full status of the generic payload internal buffer:."]
        #[inline(always)]
        pub const fn pbf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer full This bit indicates the full status of the generic payload internal buffer:."]
        #[inline(always)]
        pub fn set_pbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("cmdbe", &self.cmdbe())
                .field("cmdbf", &self.cmdbf())
                .field("pbe", &self.pbe())
                .field("pbf", &self.pbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gpsr {{ cmdfe: {=bool:?}, cmdff: {=bool:?}, pwrfe: {=bool:?}, pwrff: {=bool:?}, prdfe: {=bool:?}, prdff: {=bool:?}, rcb: {=bool:?}, cmdbe: {=bool:?}, cmdbf: {=bool:?}, pbe: {=bool:?}, pbf: {=bool:?} }}" , self . cmdfe () , self . cmdff () , self . pwrfe () , self . pwrff () , self . prdfe () , self . prdff () , self . rcb () , self . cmdbe () , self . cmdbf () , self . pbe () , self . pbf ())
        }
    }
    #[doc = "DSI Host generic VCID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gvcidr(pub u32);
    impl Gvcidr {
        #[doc = "Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
        #[inline(always)]
        pub const fn vcidrx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
        #[inline(always)]
        pub fn set_vcidrx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
        #[inline(always)]
        pub const fn vcidtx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
        #[inline(always)]
        pub fn set_vcidtx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
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
            f.debug_struct("Gvcidr")
                .field("vcidrx", &self.vcidrx())
                .field("vcidtx", &self.vcidtx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gvcidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gvcidr {{ vcidrx: {=u8:?}, vcidtx: {=u8:?} }}",
                self.vcidrx(),
                self.vcidtx()
            )
        }
    }
    #[doc = "DSI Host interrupt enable register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier0(pub u32);
    impl Ier0 {
        #[doc = "Acknowledge error 0 interrupt enable This bit enables the interrupt generation on acknowledge error 0."]
        #[inline(always)]
        pub const fn ae0ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 0 interrupt enable This bit enables the interrupt generation on acknowledge error 0."]
        #[inline(always)]
        pub fn set_ae0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge error 1 interrupt enable This bit enables the interrupt generation on acknowledge error 1."]
        #[inline(always)]
        pub const fn ae1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 1 interrupt enable This bit enables the interrupt generation on acknowledge error 1."]
        #[inline(always)]
        pub fn set_ae1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acknowledge error 2 interrupt enable This bit enables the interrupt generation on acknowledge error 2."]
        #[inline(always)]
        pub const fn ae2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 2 interrupt enable This bit enables the interrupt generation on acknowledge error 2."]
        #[inline(always)]
        pub fn set_ae2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Acknowledge error 3 interrupt enable This bit enables the interrupt generation on acknowledge error 3."]
        #[inline(always)]
        pub const fn ae3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 3 interrupt enable This bit enables the interrupt generation on acknowledge error 3."]
        #[inline(always)]
        pub fn set_ae3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Acknowledge error 4 interrupt enable This bit enables the interrupt generation on acknowledge error 4."]
        #[inline(always)]
        pub const fn ae4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 4 interrupt enable This bit enables the interrupt generation on acknowledge error 4."]
        #[inline(always)]
        pub fn set_ae4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Acknowledge error 5 interrupt enable This bit enables the interrupt generation on acknowledge error 5."]
        #[inline(always)]
        pub const fn ae5ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 5 interrupt enable This bit enables the interrupt generation on acknowledge error 5."]
        #[inline(always)]
        pub fn set_ae5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Acknowledge error 6 interrupt enable This bit enables the interrupt generation on acknowledge error 6."]
        #[inline(always)]
        pub const fn ae6ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 6 interrupt enable This bit enables the interrupt generation on acknowledge error 6."]
        #[inline(always)]
        pub fn set_ae6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Acknowledge error 7 interrupt enable This bit enables the interrupt generation on acknowledge error 7."]
        #[inline(always)]
        pub const fn ae7ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 7 interrupt enable This bit enables the interrupt generation on acknowledge error 7."]
        #[inline(always)]
        pub fn set_ae7ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Acknowledge error 8 interrupt enable This bit enables the interrupt generation on acknowledge error 8."]
        #[inline(always)]
        pub const fn ae8ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 8 interrupt enable This bit enables the interrupt generation on acknowledge error 8."]
        #[inline(always)]
        pub fn set_ae8ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Acknowledge error 9 interrupt enable This bit enables the interrupt generation on acknowledge error 9."]
        #[inline(always)]
        pub const fn ae9ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 9 interrupt enable This bit enables the interrupt generation on acknowledge error 9."]
        #[inline(always)]
        pub fn set_ae9ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge error 10 interrupt enable This bit enables the interrupt generation on acknowledge error 10."]
        #[inline(always)]
        pub const fn ae10ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 10 interrupt enable This bit enables the interrupt generation on acknowledge error 10."]
        #[inline(always)]
        pub fn set_ae10ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge error 11 interrupt enable This bit enables the interrupt generation on acknowledge error 11."]
        #[inline(always)]
        pub const fn ae11ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 11 interrupt enable This bit enables the interrupt generation on acknowledge error 11."]
        #[inline(always)]
        pub fn set_ae11ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Acknowledge error 12 interrupt enable This bit enables the interrupt generation on acknowledge error 12."]
        #[inline(always)]
        pub const fn ae12ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 12 interrupt enable This bit enables the interrupt generation on acknowledge error 12."]
        #[inline(always)]
        pub fn set_ae12ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Acknowledge error 13 interrupt enable This bit enables the interrupt generation on acknowledge error 13."]
        #[inline(always)]
        pub const fn ae13ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 13 interrupt enable This bit enables the interrupt generation on acknowledge error 13."]
        #[inline(always)]
        pub fn set_ae13ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Acknowledge error 14 interrupt enable This bit enables the interrupt generation on acknowledge error 14."]
        #[inline(always)]
        pub const fn ae14ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 14 interrupt enable This bit enables the interrupt generation on acknowledge error 14."]
        #[inline(always)]
        pub fn set_ae14ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Acknowledge error 15 interrupt enable This bit enables the interrupt generation on acknowledge error 15."]
        #[inline(always)]
        pub const fn ae15ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 15 interrupt enable This bit enables the interrupt generation on acknowledge error 15."]
        #[inline(always)]
        pub fn set_ae15ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PHY error 0 interrupt enable This bit enables the interrupt generation on PHY error 0."]
        #[inline(always)]
        pub const fn pe0ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 0 interrupt enable This bit enables the interrupt generation on PHY error 0."]
        #[inline(always)]
        pub fn set_pe0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY error 1 interrupt enable This bit enables the interrupt generation on PHY error 1."]
        #[inline(always)]
        pub const fn pe1ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 1 interrupt enable This bit enables the interrupt generation on PHY error 1."]
        #[inline(always)]
        pub fn set_pe1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY error 2 interrupt enable This bit enables the interrupt generation on PHY error 2."]
        #[inline(always)]
        pub const fn pe2ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 2 interrupt enable This bit enables the interrupt generation on PHY error 2."]
        #[inline(always)]
        pub fn set_pe2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PHY error 3 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub const fn pe3ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 3 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub fn set_pe3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PHY error 4 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub const fn pe4ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 4 interrupt enable This bit enables the interrupt generation on PHY error 4."]
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
    #[doc = "DSI Host interrupt enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier1(pub u32);
    impl Ier1 {
        #[doc = "Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission."]
        #[inline(always)]
        pub const fn tohstxie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission."]
        #[inline(always)]
        pub fn set_tohstxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception."]
        #[inline(always)]
        pub const fn tolprxie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception."]
        #[inline(always)]
        pub fn set_tolprxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error."]
        #[inline(always)]
        pub const fn eccseie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error."]
        #[inline(always)]
        pub fn set_eccseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error."]
        #[inline(always)]
        pub const fn eccmeie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error."]
        #[inline(always)]
        pub fn set_eccmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC error interrupt enable This bit enables the interrupt generation on CRC error."]
        #[inline(always)]
        pub const fn crceie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error interrupt enable This bit enables the interrupt generation on CRC error."]
        #[inline(always)]
        pub fn set_crceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Packet size error interrupt enable This bit enables the interrupt generation on packet size error."]
        #[inline(always)]
        pub const fn pseie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Packet size error interrupt enable This bit enables the interrupt generation on packet size error."]
        #[inline(always)]
        pub fn set_pseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EoTp error interrupt enable This bit enables the interrupt generation on EoTp error."]
        #[inline(always)]
        pub const fn eotpeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp error interrupt enable This bit enables the interrupt generation on EoTp error."]
        #[inline(always)]
        pub fn set_eotpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error."]
        #[inline(always)]
        pub const fn lpwreie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error."]
        #[inline(always)]
        pub fn set_lpwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error."]
        #[inline(always)]
        pub const fn gcwreie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error."]
        #[inline(always)]
        pub fn set_gcwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error."]
        #[inline(always)]
        pub const fn gpwreie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error."]
        #[inline(always)]
        pub fn set_gpwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error."]
        #[inline(always)]
        pub const fn gptxeie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error."]
        #[inline(always)]
        pub fn set_gptxeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error."]
        #[inline(always)]
        pub const fn gprdeie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error."]
        #[inline(always)]
        pub fn set_gprdeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error."]
        #[inline(always)]
        pub const fn gprxeie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error."]
        #[inline(always)]
        pub fn set_gprxeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Payload buffer underflow error interrupt enable This bit enables the interrupt generation on payload buffer underflow error."]
        #[inline(always)]
        pub const fn pbueie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer underflow error interrupt enable This bit enables the interrupt generation on payload buffer underflow error."]
        #[inline(always)]
        pub fn set_pbueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("pbueie", &self.pbueie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier1 {{ tohstxie: {=bool:?}, tolprxie: {=bool:?}, eccseie: {=bool:?}, eccmeie: {=bool:?}, crceie: {=bool:?}, pseie: {=bool:?}, eotpeie: {=bool:?}, lpwreie: {=bool:?}, gcwreie: {=bool:?}, gpwreie: {=bool:?}, gptxeie: {=bool:?}, gprdeie: {=bool:?}, gprxeie: {=bool:?}, pbueie: {=bool:?} }}" , self . tohstxie () , self . tolprxie () , self . eccseie () , self . eccmeie () , self . crceie () , self . pseie () , self . eotpeie () , self . lpwreie () , self . gcwreie () , self . gpwreie () , self . gptxeie () , self . gprdeie () , self . gprxeie () , self . pbueie ())
        }
    }
    #[doc = "DSI Host interrupt and status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr0(pub u32);
    impl Isr0 {
        #[doc = "Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0."]
        #[inline(always)]
        pub const fn pe0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0."]
        #[inline(always)]
        pub fn set_pe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0."]
        #[inline(always)]
        pub const fn pe1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0."]
        #[inline(always)]
        pub fn set_pe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY error 2 This bit indicates the ErrControl error from lane 0."]
        #[inline(always)]
        pub const fn pe2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 2 This bit indicates the ErrControl error from lane 0."]
        #[inline(always)]
        pub fn set_pe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0."]
        #[inline(always)]
        pub const fn pe3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0."]
        #[inline(always)]
        pub fn set_pe3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0."]
        #[inline(always)]
        pub const fn pe4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0."]
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
    #[doc = "DSI Host interrupt and status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub const fn tohstx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub fn set_tohstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub const fn tolprx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub fn set_tolprx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet."]
        #[inline(always)]
        pub const fn eccse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet."]
        #[inline(always)]
        pub fn set_eccse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet."]
        #[inline(always)]
        pub const fn eccme(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet."]
        #[inline(always)]
        pub fn set_eccme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC error This bit indicates that the CRC error is detected in the received packet payload."]
        #[inline(always)]
        pub const fn crce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error This bit indicates that the CRC error is detected in the received packet payload."]
        #[inline(always)]
        pub fn set_crce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Packet size error This bit indicates that the packet size error is detected during the packet reception."]
        #[inline(always)]
        pub const fn pse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Packet size error This bit indicates that the packet size error is detected during the packet reception."]
        #[inline(always)]
        pub fn set_pse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub const fn eotpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub fn set_eotpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted."]
        #[inline(always)]
        pub const fn lpwre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted."]
        #[inline(always)]
        pub fn set_lpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written."]
        #[inline(always)]
        pub const fn gcwre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written."]
        #[inline(always)]
        pub fn set_gcwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written."]
        #[inline(always)]
        pub const fn gpwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written."]
        #[inline(always)]
        pub fn set_gpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent."]
        #[inline(always)]
        pub const fn gptxe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent."]
        #[inline(always)]
        pub fn set_gptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted."]
        #[inline(always)]
        pub const fn gprde(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted."]
        #[inline(always)]
        pub fn set_gprde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted."]
        #[inline(always)]
        pub const fn gprxe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted."]
        #[inline(always)]
        pub fn set_gprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode."]
        #[inline(always)]
        pub const fn pbue(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode."]
        #[inline(always)]
        pub fn set_pbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("pbue", &self.pbue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr1 {{ tohstx: {=bool:?}, tolprx: {=bool:?}, eccse: {=bool:?}, eccme: {=bool:?}, crce: {=bool:?}, pse: {=bool:?}, eotpe: {=bool:?}, lpwre: {=bool:?}, gcwre: {=bool:?}, gpwre: {=bool:?}, gptxe: {=bool:?}, gprde: {=bool:?}, gprxe: {=bool:?}, pbue: {=bool:?} }}" , self . tohstx () , self . tolprx () , self . eccse () , self . eccme () , self . crce () , self . pse () , self . eotpe () , self . lpwre () , self . gcwre () , self . gpwre () , self . gptxe () , self . gprde () , self . gprxe () , self . pbue ())
        }
    }
    #[doc = "DSI Host LTDC current color coding register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcccr(pub u32);
    impl Lcccr {
        #[doc = "Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit."]
        #[inline(always)]
        pub const fn colc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit."]
        #[inline(always)]
        pub fn set_colc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations."]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations."]
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
    #[doc = "DSI Host LTDC command configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lccr(pub u32);
    impl Lccr {
        #[doc = "Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
        #[inline(always)]
        pub const fn cmdsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
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
    #[doc = "DSI Host LTDC color coding register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcolcr(pub u32);
    impl Lcolcr {
        #[doc = "Color coding This field configures the DPI color coding. Others: Reserved."]
        #[inline(always)]
        pub const fn colc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Color coding This field configures the DPI color coding. Others: Reserved."]
        #[inline(always)]
        pub fn set_colc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration."]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration."]
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
    #[doc = "DSI Host LTDC current VCID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcvcidr(pub u32);
    impl Lcvcidr {
        #[doc = "Virtual channel ID This field returns the virtual channel ID for the LTDC interface."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID This field returns the virtual channel ID for the LTDC interface."]
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
    #[doc = "DSI Host LTDC polarity configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpcr(pub u32);
    impl Lpcr {
        #[doc = "Data enable polarity This bit configures the polarity of data enable pin."]
        #[inline(always)]
        pub const fn dep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data enable polarity This bit configures the polarity of data enable pin."]
        #[inline(always)]
        pub fn set_dep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VSYNC polarity This bit configures the polarity of VSYNC pin."]
        #[inline(always)]
        pub const fn vsp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC polarity This bit configures the polarity of VSYNC pin."]
        #[inline(always)]
        pub fn set_vsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSYNC polarity This bit configures the polarity of HSYNC pin."]
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSYNC polarity This bit configures the polarity of HSYNC pin."]
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
    #[doc = "DSI Host low-power mode current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmccr(pub u32);
    impl Lpmccr {
        #[doc = "VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub const fn vlpsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub fn set_vlpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub const fn lpsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode."]
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
    #[doc = "DSI Host low-power mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmcr(pub u32);
    impl Lpmcr {
        #[doc = "VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
        #[inline(always)]
        pub const fn vlpsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
        #[inline(always)]
        pub fn set_vlpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
        #[inline(always)]
        pub const fn lpsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
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
    #[doc = "DSI Host LTDC VCID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvcidr(pub u32);
    impl Lvcidr {
        #[doc = "Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic."]
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
    #[doc = "DSI Host mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Command mode This bit configures the DSI Host in either video or command mode."]
        #[inline(always)]
        pub const fn cmdm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command mode This bit configures the DSI Host in either video or command mode."]
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
    #[doc = "DSI Host PHY configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pconfr(pub u32);
    impl Pconfr {
        #[doc = "Number of lanes This field configures the number of active data lanes: Others: Reserved."]
        #[inline(always)]
        pub const fn nl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Number of lanes This field configures the number of active data lanes: Others: Reserved."]
        #[inline(always)]
        pub fn set_nl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
        #[inline(always)]
        pub const fn sw_time(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
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
    #[doc = "DSI Host protocol configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "EoTp transmission enable This bit enables the EoTP transmission."]
        #[inline(always)]
        pub const fn ettxe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp transmission enable This bit enables the EoTP transmission."]
        #[inline(always)]
        pub fn set_ettxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EoTp reception enable This bit enables the EoTp reception."]
        #[inline(always)]
        pub const fn etrxe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp reception enable This bit enables the EoTp reception."]
        #[inline(always)]
        pub fn set_etrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
        #[inline(always)]
        pub const fn btae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
        #[inline(always)]
        pub fn set_btae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC reception enable This bit enables the ECC reception, error correction and reporting."]
        #[inline(always)]
        pub const fn eccrxe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC reception enable This bit enables the ECC reception, error correction and reporting."]
        #[inline(always)]
        pub fn set_eccrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC reception enable This bit enables the CRC reception and error reporting."]
        #[inline(always)]
        pub const fn crcrxe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reception enable This bit enables the CRC reception and error reporting."]
        #[inline(always)]
        pub fn set_crcrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
        #[inline(always)]
        pub const fn ettxlpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
        #[inline(always)]
        pub fn set_ettxlpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("ettxlpe", &self.ettxlpe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pcr {{ ettxe: {=bool:?}, etrxe: {=bool:?}, btae: {=bool:?}, eccrxe: {=bool:?}, crcrxe: {=bool:?}, ettxlpe: {=bool:?} }}" , self . ettxe () , self . etrxe () , self . btae () , self . eccrxe () , self . crcrxe () , self . ettxlpe ())
        }
    }
    #[doc = "DSI Host PHY control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pctlr(pub u32);
    impl Pctlr {
        #[doc = "Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state."]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state."]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clock enable This bit enables the D-PHY clock lane module:."]
        #[inline(always)]
        pub const fn cke(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable This bit enables the D-PHY clock lane module:."]
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
    #[doc = "DSI Host PHY status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "PHY direction This bit indicates the status of phydirection D-PHY signal."]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PHY direction This bit indicates the status of phydirection D-PHY signal."]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal."]
        #[inline(always)]
        pub const fn pssc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal."]
        #[inline(always)]
        pub fn set_pssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal."]
        #[inline(always)]
        pub const fn uanc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal."]
        #[inline(always)]
        pub fn set_uanc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal."]
        #[inline(always)]
        pub const fn pss0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_pss0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal."]
        #[inline(always)]
        pub const fn uan0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_uan0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal."]
        #[inline(always)]
        pub const fn rue0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_rue0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal."]
        #[inline(always)]
        pub const fn pss1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal."]
        #[inline(always)]
        pub fn set_pss1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal."]
        #[inline(always)]
        pub const fn uan1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal."]
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
    #[doc = "DSI Host PHY TX triggers configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pttcr(pub u32);
    impl Pttcr {
        #[doc = "Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time."]
        #[inline(always)]
        pub const fn tx_trig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time."]
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
    #[doc = "DSI Host PHY ULPS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucr(pub u32);
    impl Pucr {
        #[doc = "ULPS request on clock lane ULPS mode request on clock lane."]
        #[inline(always)]
        pub const fn urcl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS request on clock lane ULPS mode request on clock lane."]
        #[inline(always)]
        pub fn set_urcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ULPS exit on clock lane ULPS mode exit on clock lane."]
        #[inline(always)]
        pub const fn uecl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS exit on clock lane ULPS mode exit on clock lane."]
        #[inline(always)]
        pub fn set_uecl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ULPS request on data lane ULPS mode request on all active data lanes."]
        #[inline(always)]
        pub const fn urdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS request on data lane ULPS mode request on all active data lanes."]
        #[inline(always)]
        pub fn set_urdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ULPS exit on data lane ULPS mode exit on all active data lanes."]
        #[inline(always)]
        pub const fn uedl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS exit on data lane ULPS mode exit on all active data lanes."]
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
    #[doc = "DSI Host timeout counter configuration register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr0(pub u32);
    impl Tccr0 {
        #[doc = "Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles)."]
        #[inline(always)]
        pub const fn lprx_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles)."]
        #[inline(always)]
        pub fn set_lprx_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one LINE data transmission *Â (1Â +Â 10%)."]
        #[inline(always)]
        pub const fn hstx_tocnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one LINE data transmission *Â (1Â +Â 10%)."]
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
    #[doc = "DSI Host timeout counter configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr1(pub u32);
    impl Tccr1 {
        #[doc = "High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn hsrd_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
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
    #[doc = "DSI Host timeout counter configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr2(pub u32);
    impl Tccr2 {
        #[doc = "Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn lprd_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
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
    #[doc = "DSI Host timeout counter configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr3(pub u32);
    impl Tccr3 {
        #[doc = "High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn hswr_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_hswr_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
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
    #[doc = "DSI Host timeout counter configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr4(pub u32);
    impl Tccr4 {
        #[doc = "Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn lpwr_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_lpwr_tocnt(&mut self, val: u16) {
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
            f.debug_struct("Tccr4").field("lpwr_tocnt", &self.lpwr_tocnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tccr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tccr4 {{ lpwr_tocnt: {=u16:?} }}", self.lpwr_tocnt())
        }
    }
    #[doc = "DSI Host timeout counter configuration register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr5(pub u32);
    impl Tccr5 {
        #[doc = "Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the Dâ\u{80}\u{91}PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn bta_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the Dâ\u{80}\u{91}PHY enters the Stop state and causes no interrupts."]
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
    #[doc = "DSI Host video chunks current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vcccr(pub u32);
    impl Vcccr {
        #[doc = "Number of chunks This field returns the number of chunks being transmitted during a line period."]
        #[inline(always)]
        pub const fn numc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of chunks This field returns the number of chunks being transmitted during a line period."]
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
    #[doc = "DSI Host video chunks configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vccr(pub u32);
    impl Vccr {
        #[doc = "Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line."]
        #[inline(always)]
        pub const fn numc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line."]
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
    #[doc = "DSI Host video HBP current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhbpccr(pub u32);
    impl Vhbpccr {
        #[doc = "Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles."]
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
    #[doc = "DSI Host video HBP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhbpcr(pub u32);
    impl Vhbpcr {
        #[doc = "Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles."]
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
    #[doc = "DSI Host video HSA current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhsaccr(pub u32);
    impl Vhsaccr {
        #[doc = "Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles."]
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
    #[doc = "DSI Host video HSA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhsacr(pub u32);
    impl Vhsacr {
        #[doc = "Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles."]
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
    #[doc = "DSI Host video line current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vlccr(pub u32);
    impl Vlccr {
        #[doc = "Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
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
    #[doc = "DSI Host video line configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vlcr(pub u32);
    impl Vlcr {
        #[doc = "Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
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
    #[doc = "DSI Host video mode current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmccr(pub u32);
    impl Vmccr {
        #[doc = "Video mode type This field returns the current video mode transmission type: 1x: Burst mode."]
        #[inline(always)]
        pub const fn vmt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Video mode type This field returns the current video mode transmission type: 1x: Burst mode."]
        #[inline(always)]
        pub fn set_vmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub const fn lpvsae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvbpe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvfpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub const fn lpvae(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub const fn lphbpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub const fn lphfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub const fn fbtaae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub fn set_fbtaae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power command enable This bit returns the current command transmission state in low-power mode."]
        #[inline(always)]
        pub const fn lpce(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power command enable This bit returns the current command transmission state in low-power mode."]
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
    #[doc = "DSI Host video mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmcr(pub u32);
    impl Vmcr {
        #[doc = "Video mode type This field configures the video mode transmission type : 1x: Burst mode."]
        #[inline(always)]
        pub const fn vmt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Video mode type This field configures the video mode transmission type : 1x: Burst mode."]
        #[inline(always)]
        pub fn set_vmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub const fn lpvsae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvbpe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvfpe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub const fn lpvae(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub const fn lphbpe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub const fn lphfpe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub const fn fbtaae(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub fn set_fbtaae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Low-power command enable This bit enables the command transmission only in low-power mode."]
        #[inline(always)]
        pub const fn lpce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power command enable This bit enables the command transmission only in low-power mode."]
        #[inline(always)]
        pub fn set_lpce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Pattern generator enable This bit enables the video mode pattern generator."]
        #[inline(always)]
        pub const fn pge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern generator enable This bit enables the video mode pattern generator."]
        #[inline(always)]
        pub fn set_pge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pattern generator mode This bit configures the pattern generator mode."]
        #[inline(always)]
        pub const fn pgm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern generator mode This bit configures the pattern generator mode."]
        #[inline(always)]
        pub fn set_pgm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Pattern generator orientation This bit configures the color bar orientation."]
        #[inline(always)]
        pub const fn pgo(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern generator orientation This bit configures the color bar orientation."]
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
    #[doc = "DSI Host video null packet current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vnpccr(pub u32);
    impl Vnpccr {
        #[doc = "Null packet size This field returns the number of bytes inside a null packet."]
        #[inline(always)]
        pub const fn npsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Null packet size This field returns the number of bytes inside a null packet."]
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
    #[doc = "DSI Host video null packet configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vnpcr(pub u32);
    impl Vnpcr {
        #[doc = "Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets."]
        #[inline(always)]
        pub const fn npsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets."]
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
    #[doc = "DSI Host video packet current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vpccr(pub u32);
    impl Vpccr {
        #[doc = "Video packet size This field returns the number of pixels in a single video packet."]
        #[inline(always)]
        pub const fn vpsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Video packet size This field returns the number of pixels in a single video packet."]
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
    #[doc = "DSI Host video packet configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vpcr(pub u32);
    impl Vpcr {
        #[doc = "Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
        #[inline(always)]
        pub const fn vpsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
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
    #[doc = "DSI Host version register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vr(pub u32);
    impl Vr {
        #[doc = "Version of the DSI Host This read-only register contains the version of the DSI Host."]
        #[inline(always)]
        pub const fn version(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Version of the DSI Host This read-only register contains the version of the DSI Host."]
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
    #[doc = "DSI Host video shadow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vscr(pub u32);
    impl Vscr {
        #[doc = "Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
        #[inline(always)]
        pub const fn ur(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
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
    #[doc = "DSI Host video VA current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvaccr(pub u32);
    impl Vvaccr {
        #[doc = "Vertical active duration This field returns the current vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn va(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical active duration This field returns the current vertical active period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvacr(pub u32);
    impl Vvacr {
        #[doc = "Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn va(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VBP current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvbpccr(pub u32);
    impl Vvbpccr {
        #[doc = "Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VBP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvbpcr(pub u32);
    impl Vvbpcr {
        #[doc = "Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VFP current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvfpccr(pub u32);
    impl Vvfpccr {
        #[doc = "Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vfp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VFP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvfpcr(pub u32);
    impl Vvfpcr {
        #[doc = "Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vfp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VSA current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvsaccr(pub u32);
    impl Vvsaccr {
        #[doc = "Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines."]
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
    #[doc = "DSI Host video VSA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvsacr(pub u32);
    impl Vvsacr {
        #[doc = "Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
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
    #[doc = "DSI Wrapper configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wcfgr(pub u32);
    impl Wcfgr {
        #[doc = "DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn dsim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_dsim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub const fn colmux(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub fn set_colmux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn tesrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_tesrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn tepol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_tepol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn ar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_ar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
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
    #[doc = "DSI Wrapper control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wcr(pub u32);
    impl Wcr {
        #[doc = "Color mode This bit controls the display color mode in video mode."]
        #[inline(always)]
        pub const fn colm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Color mode This bit controls the display color mode in video mode."]
        #[inline(always)]
        pub fn set_colm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Shutdown This bit controls the display shutdown in video mode."]
        #[inline(always)]
        pub const fn shtdn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Shutdown This bit controls the display shutdown in video mode."]
        #[inline(always)]
        pub fn set_shtdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DSI enable This bit enables the DSI Wrapper."]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DSI enable This bit enables the DSI Wrapper."]
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
    #[doc = "DSI Wrapper interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wier(pub u32);
    impl Wier {
        #[doc = "Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of refresh interrupt enable This bit enables the end of refresh interrupt."]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of refresh interrupt enable This bit enables the end of refresh interrupt."]
        #[inline(always)]
        pub fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PLL lock interrupt enable This bit enables the PLL lock interrupt."]
        #[inline(always)]
        pub const fn plllie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLL lock interrupt enable This bit enables the PLL lock interrupt."]
        #[inline(always)]
        pub fn set_plllie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
        #[inline(always)]
        pub const fn plluie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
        #[inline(always)]
        pub fn set_plluie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wier {{ teie: {=bool:?}, erie: {=bool:?}, plllie: {=bool:?}, plluie: {=bool:?} }}",
                self.teie(),
                self.erie(),
                self.plllie(),
                self.plluie()
            )
        }
    }
    #[doc = "DSI Wrapper interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wifcr(pub u32);
    impl Wifcr {
        #[doc = "Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cteif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cerif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cerif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cplllif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cplllif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cplluif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cplluif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wifcr {{ cteif: {=bool:?}, cerif: {=bool:?}, cplllif: {=bool:?}, cplluif: {=bool:?} }}",
                self.cteif(),
                self.cerif(),
                self.cplllif(),
                self.cplluif()
            )
        }
    }
    #[doc = "DSI Wrapper interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wisr(pub u32);
    impl Wisr {
        #[doc = "Tearing effect interrupt flag This bit is set when a tearing effect event occurs."]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing effect interrupt flag This bit is set when a tearing effect event occurs."]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished."]
        #[inline(always)]
        pub const fn erif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished."]
        #[inline(always)]
        pub fn set_erif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked."]
        #[inline(always)]
        pub const fn pllls(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked."]
        #[inline(always)]
        pub fn set_pllls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLL lock interrupt flag This bit is set when the PLL becomes locked."]
        #[inline(always)]
        pub const fn plllif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLL lock interrupt flag This bit is set when the PLL becomes locked."]
        #[inline(always)]
        pub fn set_plllif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PLL unlock interrupt flag This bit is set when the PLL becomes unlocked."]
        #[inline(always)]
        pub const fn plluif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PLL unlock interrupt flag This bit is set when the PLL becomes unlocked."]
        #[inline(always)]
        pub fn set_plluif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wisr {{ teif: {=bool:?}, erif: {=bool:?}, busy: {=bool:?}, pllls: {=bool:?}, plllif: {=bool:?}, plluif: {=bool:?} }}" , self . teif () , self . erif () , self . busy () , self . pllls () , self . plllif () , self . plluif ())
        }
    }
    #[doc = "DSI Wrapper PHY configuration register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr0(pub u32);
    impl Wpcr0 {
        #[doc = "Swap clock lane pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub const fn swcl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Swap clock lane pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub fn set_swcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Swap data lane 0 pins This bit swaps the pins on data lane 0."]
        #[inline(always)]
        pub const fn swdl0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Swap data lane 0 pins This bit swaps the pins on data lane 0."]
        #[inline(always)]
        pub fn set_swdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Swap data lane 1 pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub const fn swdl1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Swap data lane 1 pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub fn set_swdl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub const fn ftxsmcl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub fn set_ftxsmcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub const fn ftxsmdl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub fn set_ftxsmdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("swcl", &self.swcl())
                .field("swdl0", &self.swdl0())
                .field("swdl1", &self.swdl1())
                .field("ftxsmcl", &self.ftxsmcl())
                .field("ftxsmdl", &self.ftxsmdl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpcr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wpcr0 {{ swcl: {=bool:?}, swdl0: {=bool:?}, swdl1: {=bool:?}, ftxsmcl: {=bool:?}, ftxsmdl: {=bool:?} }}" , self . swcl () , self . swdl0 () , self . swdl1 () , self . ftxsmcl () , self . ftxsmdl ())
        }
    }
    #[doc = "DSI Wrapper regulator and PLL control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpcr(pub u32);
    impl Wrpcr {
        #[doc = "PLL enable This bit enables the D-PHY PLL."]
        #[inline(always)]
        pub const fn pllen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable This bit enables the D-PHY PLL."]
        #[inline(always)]
        pub fn set_pllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2."]
        #[inline(always)]
        pub const fn ndiv(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x01ff;
            val as u16
        }
        #[doc = "PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2."]
        #[inline(always)]
        pub fn set_ndiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
        }
        #[doc = "PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511."]
        #[inline(always)]
        pub const fn idf(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x01ff;
            val as u16
        }
        #[doc = "PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511."]
        #[inline(always)]
        pub fn set_idf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
        }
        #[doc = "PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511."]
        #[inline(always)]
        pub const fn odf(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x01ff;
            val as u16
        }
        #[doc = "PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511."]
        #[inline(always)]
        pub fn set_odf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wrpcr {{ pllen: {=bool:?}, ndiv: {=u16:?}, idf: {=u16:?}, odf: {=u16:?} }}",
                self.pllen(),
                self.ndiv(),
                self.idf(),
                self.odf()
            )
        }
    }
}
