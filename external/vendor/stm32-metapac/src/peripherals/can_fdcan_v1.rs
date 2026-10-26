#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Controller area network with flexible data rate (FD)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcan {
    ptr: *mut u8,
}
unsafe impl Send for Fdcan {}
unsafe impl Sync for Fdcan {}
impl Fdcan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FDCAN core release register"]
    #[inline(always)]
    pub const fn crel(self) -> crate::common::Reg<regs::Crel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FDCAN endian register"]
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<regs::Endn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FDCAN data bit timing and prescaler register"]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FDCAN test register"]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FDCAN RAM watchdog register"]
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<regs::Rwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "FDCAN CC control register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FDCAN nominal bit timing and prescaler register"]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FDCAN timestamp counter configuration register"]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FDCAN timestamp counter value register"]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FDCAN timeout counter configuration register"]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FDCAN timeout counter value register"]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::Tocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FDCAN error counter register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FDCAN protocol status register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FDCAN transmitter delay compensation register"]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FDCAN interrupt register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FDCAN interrupt enable register"]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FDCAN interrupt line select register"]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "FDCAN interrupt line enable register"]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "FDCAN global filter configuration register"]
    #[inline(always)]
    pub const fn rxgfc(self) -> crate::common::Reg<regs::Rxgfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FDCAN extended ID and mask register"]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FDCAN high-priority message status register"]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::Hpms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FDCAN Rx FIFO X status register"]
    #[inline(always)]
    pub const fn rxfs(self, n: usize) -> crate::common::Reg<regs::Rxfs, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "CAN Rx FIFO X acknowledge register"]
    #[inline(always)]
    pub const fn rxfa(self, n: usize) -> crate::common::Reg<regs::Rxfa, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "FDCAN Tx buffer configuration register"]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FDCAN Tx FIFO/queue status register"]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::Txfqs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FDCAN Tx buffer request pending register"]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::Txbrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "FDCAN Tx buffer add request register"]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "FDCAN Tx buffer cancellation request register"]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "FDCAN Tx buffer transmission occurred register"]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::Txbto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "FDCAN Tx buffer cancellation finished register"]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::Txbcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "FDCAN Tx buffer transmission interrupt enable register"]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "FDCAN Tx event FIFO status register"]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::Txefs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "FDCAN Tx event FIFO acknowledge register"]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn ckdiv(self) -> crate::common::Reg<regs::Ckdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
}
pub mod regs {
    #[doc = "FDCAN CC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "Initialization"]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization"]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Configuration change enable"]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration change enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ASM restricted operation mode. The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time"]
        #[inline(always)]
        pub const fn asm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ASM restricted operation mode. The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time"]
        #[inline(always)]
        pub fn set_asm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock stop acknowledge"]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clock stop acknowledge"]
        #[inline(always)]
        pub fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock stop request"]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clock stop request"]
        #[inline(always)]
        pub fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bus monitoring mode. Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time"]
        #[inline(always)]
        pub const fn mon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bus monitoring mode. Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable automatic retransmission"]
        #[inline(always)]
        pub const fn dar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable automatic retransmission"]
        #[inline(always)]
        pub fn set_dar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Test mode enable"]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Test mode enable"]
        #[inline(always)]
        pub fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FD operation enable"]
        #[inline(always)]
        pub const fn fdoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FD operation enable"]
        #[inline(always)]
        pub fn set_fdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FDCAN bit rate switching"]
        #[inline(always)]
        pub const fn brse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN bit rate switching"]
        #[inline(always)]
        pub fn set_brse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Protocol exception handling disable"]
        #[inline(always)]
        pub const fn pxhd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol exception handling disable"]
        #[inline(always)]
        pub fn set_pxhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Edge filtering during bus integration"]
        #[inline(always)]
        pub const fn efbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Edge filtering during bus integration"]
        #[inline(always)]
        pub fn set_efbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame"]
        #[inline(always)]
        pub const fn txp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame"]
        #[inline(always)]
        pub fn set_txp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Non ISO operation. If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0"]
        #[inline(always)]
        pub const fn niso(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Non ISO operation. If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0"]
        #[inline(always)]
        pub fn set_niso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cccr {
        #[inline(always)]
        fn default() -> Cccr {
            Cccr(0)
        }
    }
    impl core::fmt::Debug for Cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccr")
                .field("init", &self.init())
                .field("cce", &self.cce())
                .field("asm", &self.asm())
                .field("csa", &self.csa())
                .field("csr", &self.csr())
                .field("mon", &self.mon())
                .field("dar", &self.dar())
                .field("test", &self.test())
                .field("fdoe", &self.fdoe())
                .field("brse", &self.brse())
                .field("pxhd", &self.pxhd())
                .field("efbi", &self.efbi())
                .field("txp", &self.txp())
                .field("niso", &self.niso())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cccr {{ init: {=bool:?}, cce: {=bool:?}, asm: {=bool:?}, csa: {=bool:?}, csr: {=bool:?}, mon: {=bool:?}, dar: {=bool:?}, test: {=bool:?}, fdoe: {=bool:?}, brse: {=bool:?}, pxhd: {=bool:?}, efbi: {=bool:?}, txp: {=bool:?}, niso: {=bool:?} }}" , self . init () , self . cce () , self . asm () , self . csa () , self . csr () , self . mon () , self . dar () , self . test () , self . fdoe () , self . brse () , self . pxhd () , self . efbi () , self . txp () , self . niso ())
        }
    }
    #[doc = "FDCAN CFG clock divider register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckdiv(pub u32);
    impl Ckdiv {
        #[doc = "input clock divider. The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn pdiv(&self) -> super::vals::Pdiv {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Pdiv::from_bits(val as u8)
        }
        #[doc = "input clock divider. The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_pdiv(&mut self, val: super::vals::Pdiv) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ckdiv {
        #[inline(always)]
        fn default() -> Ckdiv {
            Ckdiv(0)
        }
    }
    impl core::fmt::Debug for Ckdiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckdiv").field("pdiv", &self.pdiv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckdiv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ckdiv {{ pdiv: {:?} }}", self.pdiv())
        }
    }
    #[doc = "FDCAN core release register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crel(pub u32);
    impl Crel {
        #[doc = "DAY"]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DAY"]
        #[inline(always)]
        pub fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "MON"]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "MON"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "YEAR"]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "YEAR"]
        #[inline(always)]
        pub fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "SUBSTEP"]
        #[inline(always)]
        pub const fn substep(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "SUBSTEP"]
        #[inline(always)]
        pub fn set_substep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "STEP"]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "STEP"]
        #[inline(always)]
        pub fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "REL"]
        #[inline(always)]
        pub const fn rel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "REL"]
        #[inline(always)]
        pub fn set_rel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Crel {
        #[inline(always)]
        fn default() -> Crel {
            Crel(0)
        }
    }
    impl core::fmt::Debug for Crel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crel")
                .field("day", &self.day())
                .field("mon", &self.mon())
                .field("year", &self.year())
                .field("substep", &self.substep())
                .field("step", &self.step())
                .field("rel", &self.rel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Crel {{ day: {=u8:?}, mon: {=u8:?}, year: {=u8:?}, substep: {=u8:?}, step: {=u8:?}, rel: {=u8:?} }}",
                self.day(),
                self.mon(),
                self.year(),
                self.substep(),
                self.step(),
                self.rel()
            )
        }
    }
    #[doc = "FDCAN data bit timing and prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbtp(pub u32);
    impl Dbtp {
        #[doc = "Synchronization jump width. Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
        #[inline(always)]
        pub const fn dsjw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Synchronization jump width. Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
        #[inline(always)]
        pub fn set_dsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Data time segment after sample point. Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq"]
        #[inline(always)]
        pub const fn dtseg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data time segment after sample point. Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq"]
        #[inline(always)]
        pub fn set_dtseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data time segment before sample point. Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq"]
        #[inline(always)]
        pub const fn dtseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data time segment before sample point. Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq"]
        #[inline(always)]
        pub fn set_dtseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data bit rate prescaler. The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1"]
        #[inline(always)]
        pub const fn dbrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Data bit rate prescaler. The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1"]
        #[inline(always)]
        pub fn set_dbrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Transceiver delay compensation"]
        #[inline(always)]
        pub const fn tdc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transceiver delay compensation"]
        #[inline(always)]
        pub fn set_tdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Dbtp {
        #[inline(always)]
        fn default() -> Dbtp {
            Dbtp(0)
        }
    }
    impl core::fmt::Debug for Dbtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbtp")
                .field("dsjw", &self.dsjw())
                .field("dtseg2", &self.dtseg2())
                .field("dtseg1", &self.dtseg1())
                .field("dbrp", &self.dbrp())
                .field("tdc", &self.tdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbtp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dbtp {{ dsjw: {=u8:?}, dtseg2: {=u8:?}, dtseg1: {=u8:?}, dbrp: {=u8:?}, tdc: {=bool:?} }}",
                self.dsjw(),
                self.dtseg2(),
                self.dtseg1(),
                self.dbrp(),
                self.tdc()
            )
        }
    }
    #[doc = "FDCAN error counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "Transmit error counter. Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented"]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit error counter. Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented"]
        #[inline(always)]
        pub fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Receive error counter. Actual state of the receive error counter, values between 0 and 127"]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Receive error counter. Actual state of the receive error counter, values between 0 and 127"]
        #[inline(always)]
        pub fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Receive error passive"]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive error passive"]
        #[inline(always)]
        pub fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CAN error logging. The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
        #[inline(always)]
        pub const fn cel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "CAN error logging. The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
        #[inline(always)]
        pub fn set_cel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ecr {
        #[inline(always)]
        fn default() -> Ecr {
            Ecr(0)
        }
    }
    impl core::fmt::Debug for Ecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecr")
                .field("tec", &self.tec())
                .field("rec", &self.rec())
                .field("rp", &self.rp())
                .field("cel", &self.cel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ecr {{ tec: {=u8:?}, rec: {=u8:?}, rp: {=bool:?}, cel: {=u8:?} }}",
                self.tec(),
                self.rec(),
                self.rp(),
                self.cel()
            )
        }
    }
    #[doc = "FDCAN endian register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Endn(pub u32);
    impl Endn {
        #[doc = "Endianness test value. The endianness test value is 0x8765 4321"]
        #[inline(always)]
        pub const fn etv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Endianness test value. The endianness test value is 0x8765 4321"]
        #[inline(always)]
        pub fn set_etv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Endn {
        #[inline(always)]
        fn default() -> Endn {
            Endn(0)
        }
    }
    impl core::fmt::Debug for Endn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Endn").field("etv", &self.etv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Endn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Endn {{ etv: {=u32:?} }}", self.etv())
        }
    }
    #[doc = "FDCAN high-priority message status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hpms(pub u32);
    impl Hpms {
        #[doc = "Buffer index. Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= 1"]
        #[inline(always)]
        pub const fn bidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Buffer index. Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= 1"]
        #[inline(always)]
        pub fn set_bidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Message storage indicator"]
        #[inline(always)]
        pub const fn msi(&self) -> super::vals::Msi {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Msi::from_bits(val as u8)
        }
        #[doc = "Message storage indicator"]
        #[inline(always)]
        pub fn set_msi(&mut self, val: super::vals::Msi) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Filter index. Index of matching filter element. Range is 0 to RXGFC\\[LSS\\]
- 1 or RXGFC\\[LSE\\]
- 1"]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Filter index. Index of matching filter element. Range is 0 to RXGFC\\[LSS\\]
- 1 or RXGFC\\[LSE\\]
- 1"]
        #[inline(always)]
        pub fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Filter list. Indicates the filter list of the matching filter element"]
        #[inline(always)]
        pub const fn flst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter list. Indicates the filter list of the matching filter element"]
        #[inline(always)]
        pub fn set_flst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Hpms {
        #[inline(always)]
        fn default() -> Hpms {
            Hpms(0)
        }
    }
    impl core::fmt::Debug for Hpms {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hpms")
                .field("bidx", &self.bidx())
                .field("msi", &self.msi())
                .field("fidx", &self.fidx())
                .field("flst", &self.flst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hpms {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hpms {{ bidx: {=u8:?}, msi: {:?}, fidx: {=u8:?}, flst: {=bool:?} }}",
                self.bidx(),
                self.msi(),
                self.fidx(),
                self.flst()
            )
        }
    }
    #[doc = "FDCAN interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ie(pub u32);
    impl Ie {
        #[doc = "Rx FIFO X new message interrupt enable"]
        #[inline(always)]
        pub const fn rfne(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X new message interrupt enable"]
        #[inline(always)]
        pub fn set_rfne(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X full interrupt enable"]
        #[inline(always)]
        pub const fn rffe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X full interrupt enable"]
        #[inline(always)]
        pub fn set_rffe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X message lost interrupt enable"]
        #[inline(always)]
        pub const fn rfle(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X message lost interrupt enable"]
        #[inline(always)]
        pub fn set_rfle(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High-priority message interrupt enable"]
        #[inline(always)]
        pub const fn hpme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "High-priority message interrupt enable"]
        #[inline(always)]
        pub fn set_hpme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmission completed interrupt enable"]
        #[inline(always)]
        pub const fn tce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission completed interrupt enable"]
        #[inline(always)]
        pub fn set_tce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmission cancellation finished interrupt enable"]
        #[inline(always)]
        pub const fn tcfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission cancellation finished interrupt enable"]
        #[inline(always)]
        pub fn set_tcfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tx FIFO empty interrupt enable"]
        #[inline(always)]
        pub const fn tfee(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty interrupt enable"]
        #[inline(always)]
        pub fn set_tfee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx event FIFO new entry interrupt enable"]
        #[inline(always)]
        pub const fn tefne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO new entry interrupt enable"]
        #[inline(always)]
        pub fn set_tefne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx event FIFO full interrupt enable"]
        #[inline(always)]
        pub const fn teffe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO full interrupt enable"]
        #[inline(always)]
        pub fn set_teffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx event FIFO element lost interrupt enable"]
        #[inline(always)]
        pub const fn tefle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO element lost interrupt enable"]
        #[inline(always)]
        pub fn set_tefle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timestamp wraparound interrupt enable"]
        #[inline(always)]
        pub const fn tswe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp wraparound interrupt enable"]
        #[inline(always)]
        pub fn set_tswe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Message RAM access failure interrupt enable"]
        #[inline(always)]
        pub const fn mrafe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM access failure interrupt enable"]
        #[inline(always)]
        pub fn set_mrafe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timeout occurred interrupt enable"]
        #[inline(always)]
        pub const fn tooe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout occurred interrupt enable"]
        #[inline(always)]
        pub fn set_tooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Error logging overflow interrupt enable"]
        #[inline(always)]
        pub const fn eloe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error logging overflow interrupt enable"]
        #[inline(always)]
        pub fn set_eloe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Error passive interrupt enable"]
        #[inline(always)]
        pub const fn epe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive interrupt enable"]
        #[inline(always)]
        pub fn set_epe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Warning status interrupt enable"]
        #[inline(always)]
        pub const fn ewe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Warning status interrupt enable"]
        #[inline(always)]
        pub fn set_ewe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bus_Off status enable"]
        #[inline(always)]
        pub const fn boe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off status enable"]
        #[inline(always)]
        pub fn set_boe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Watchdog interrupt enable"]
        #[inline(always)]
        pub const fn wdie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog interrupt enable"]
        #[inline(always)]
        pub fn set_wdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Protocol error in arbitration phase enable"]
        #[inline(always)]
        pub const fn peae(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in arbitration phase enable"]
        #[inline(always)]
        pub fn set_peae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Protocol error in data phase enable"]
        #[inline(always)]
        pub const fn pede(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in data phase enable"]
        #[inline(always)]
        pub fn set_pede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Access to reserved address enable"]
        #[inline(always)]
        pub const fn arae(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Access to reserved address enable"]
        #[inline(always)]
        pub fn set_arae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ie {
        #[inline(always)]
        fn default() -> Ie {
            Ie(0)
        }
    }
    impl core::fmt::Debug for Ie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ie")
                .field("rfne[0]", &self.rfne(0usize))
                .field("rfne[1]", &self.rfne(1usize))
                .field("rffe[0]", &self.rffe(0usize))
                .field("rffe[1]", &self.rffe(1usize))
                .field("rfle[0]", &self.rfle(0usize))
                .field("rfle[1]", &self.rfle(1usize))
                .field("hpme", &self.hpme())
                .field("tce", &self.tce())
                .field("tcfe", &self.tcfe())
                .field("tfee", &self.tfee())
                .field("tefne", &self.tefne())
                .field("teffe", &self.teffe())
                .field("tefle", &self.tefle())
                .field("tswe", &self.tswe())
                .field("mrafe", &self.mrafe())
                .field("tooe", &self.tooe())
                .field("eloe", &self.eloe())
                .field("epe", &self.epe())
                .field("ewe", &self.ewe())
                .field("boe", &self.boe())
                .field("wdie", &self.wdie())
                .field("peae", &self.peae())
                .field("pede", &self.pede())
                .field("arae", &self.arae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ie {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ie {{ rfne[0]: {=bool:?}, rfne[1]: {=bool:?}, rffe[0]: {=bool:?}, rffe[1]: {=bool:?}, rfle[0]: {=bool:?}, rfle[1]: {=bool:?}, hpme: {=bool:?}, tce: {=bool:?}, tcfe: {=bool:?}, tfee: {=bool:?}, tefne: {=bool:?}, teffe: {=bool:?}, tefle: {=bool:?}, tswe: {=bool:?}, mrafe: {=bool:?}, tooe: {=bool:?}, eloe: {=bool:?}, epe: {=bool:?}, ewe: {=bool:?}, boe: {=bool:?}, wdie: {=bool:?}, peae: {=bool:?}, pede: {=bool:?}, arae: {=bool:?} }}" , self . rfne (0usize) , self . rfne (1usize) , self . rffe (0usize) , self . rffe (1usize) , self . rfle (0usize) , self . rfle (1usize) , self . hpme () , self . tce () , self . tcfe () , self . tfee () , self . tefne () , self . teffe () , self . tefle () , self . tswe () , self . mrafe () , self . tooe () , self . eloe () , self . epe () , self . ewe () , self . boe () , self . wdie () , self . peae () , self . pede () , self . arae ())
        }
    }
    #[doc = "FDCAN interrupt line enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ile(pub u32);
    impl Ile {
        #[doc = "Enable interrupt line 0"]
        #[inline(always)]
        pub const fn eint0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable interrupt line 0"]
        #[inline(always)]
        pub fn set_eint0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable interrupt line 1"]
        #[inline(always)]
        pub const fn eint1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable interrupt line 1"]
        #[inline(always)]
        pub fn set_eint1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ile {
        #[inline(always)]
        fn default() -> Ile {
            Ile(0)
        }
    }
    impl core::fmt::Debug for Ile {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ile")
                .field("eint0", &self.eint0())
                .field("eint1", &self.eint1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ile {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ile {{ eint0: {=bool:?}, eint1: {=bool:?} }}",
                self.eint0(),
                self.eint1()
            )
        }
    }
    #[doc = "FDCAN interrupt line select register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ils(pub u32);
    impl Ils {
        #[doc = "RX FIFO bit grouping the following interruption. RFLL: Rx FIFO X message lost interrupt line RFFL: Rx FIFO X full interrupt line RFNL: Rx FIFO X new message interrupt line."]
        #[inline(always)]
        pub const fn rxfifo(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO bit grouping the following interruption. RFLL: Rx FIFO X message lost interrupt line RFFL: Rx FIFO X full interrupt line RFNL: Rx FIFO X new message interrupt line."]
        #[inline(always)]
        pub fn set_rxfifo(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Status message bit grouping the following interruption. TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line."]
        #[inline(always)]
        pub const fn smsg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Status message bit grouping the following interruption. TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line."]
        #[inline(always)]
        pub fn set_smsg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tx FIFO ERROR grouping the following interruption. TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line."]
        #[inline(always)]
        pub const fn tferr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO ERROR grouping the following interruption. TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line."]
        #[inline(always)]
        pub fn set_tferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt regrouping the following interruption. TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line."]
        #[inline(always)]
        pub const fn misc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt regrouping the following interruption. TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line."]
        #[inline(always)]
        pub fn set_misc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bit and line error grouping the following interruption. EPL Error passive interrupt line ELOL: Error logging overflow interrupt line."]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bit and line error grouping the following interruption. EPL Error passive interrupt line ELOL: Error logging overflow interrupt line."]
        #[inline(always)]
        pub fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Protocol error grouping the following interruption. ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line."]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error grouping the following interruption. ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line."]
        #[inline(always)]
        pub fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ils {
        #[inline(always)]
        fn default() -> Ils {
            Ils(0)
        }
    }
    impl core::fmt::Debug for Ils {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ils")
                .field("rxfifo[0]", &self.rxfifo(0usize))
                .field("rxfifo[1]", &self.rxfifo(1usize))
                .field("smsg", &self.smsg())
                .field("tferr", &self.tferr())
                .field("misc", &self.misc())
                .field("berr", &self.berr())
                .field("perr", &self.perr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ils {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ils {{ rxfifo[0]: {=bool:?}, rxfifo[1]: {=bool:?}, smsg: {=bool:?}, tferr: {=bool:?}, misc: {=bool:?}, berr: {=bool:?}, perr: {=bool:?} }}" , self . rxfifo (0usize) , self . rxfifo (1usize) , self . smsg () , self . tferr () , self . misc () , self . berr () , self . perr ())
        }
    }
    #[doc = "FDCAN interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Rx FIFO X new message"]
        #[inline(always)]
        pub const fn rfn(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X new message"]
        #[inline(always)]
        pub fn set_rfn(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X full"]
        #[inline(always)]
        pub const fn rff(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X full"]
        #[inline(always)]
        pub fn set_rff(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X message lost"]
        #[inline(always)]
        pub const fn rfl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X message lost"]
        #[inline(always)]
        pub fn set_rfl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High-priority message"]
        #[inline(always)]
        pub const fn hpm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "High-priority message"]
        #[inline(always)]
        pub fn set_hpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmission completed"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission completed"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmission cancellation finished"]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission cancellation finished"]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tx FIFO empty"]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty"]
        #[inline(always)]
        pub fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx event FIFO New Entry"]
        #[inline(always)]
        pub const fn tefn(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO New Entry"]
        #[inline(always)]
        pub fn set_tefn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx event FIFO full"]
        #[inline(always)]
        pub const fn teff(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO full"]
        #[inline(always)]
        pub fn set_teff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx event FIFO element lost"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO element lost"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timestamp wraparound"]
        #[inline(always)]
        pub const fn tsw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp wraparound"]
        #[inline(always)]
        pub fn set_tsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Message RAM access failure. The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
        #[inline(always)]
        pub const fn mraf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM access failure. The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
        #[inline(always)]
        pub fn set_mraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timeout occurred"]
        #[inline(always)]
        pub const fn too(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout occurred"]
        #[inline(always)]
        pub fn set_too(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Error logging overflow"]
        #[inline(always)]
        pub const fn elo(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error logging overflow"]
        #[inline(always)]
        pub fn set_elo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Error passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Warning status"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Warning status"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bus_Off status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Watchdog interrupt"]
        #[inline(always)]
        pub const fn wdi(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog interrupt"]
        #[inline(always)]
        pub fn set_wdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Protocol error in arbitration phase (nominal bit time is used)"]
        #[inline(always)]
        pub const fn pea(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in arbitration phase (nominal bit time is used)"]
        #[inline(always)]
        pub fn set_pea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Protocol error in data phase (data bit time is used)"]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in data phase (data bit time is used)"]
        #[inline(always)]
        pub fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Access to reserved address"]
        #[inline(always)]
        pub const fn ara(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Access to reserved address"]
        #[inline(always)]
        pub fn set_ara(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ir {
        #[inline(always)]
        fn default() -> Ir {
            Ir(0)
        }
    }
    impl core::fmt::Debug for Ir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ir")
                .field("rfn[0]", &self.rfn(0usize))
                .field("rfn[1]", &self.rfn(1usize))
                .field("rff[0]", &self.rff(0usize))
                .field("rff[1]", &self.rff(1usize))
                .field("rfl[0]", &self.rfl(0usize))
                .field("rfl[1]", &self.rfl(1usize))
                .field("hpm", &self.hpm())
                .field("tc", &self.tc())
                .field("tcf", &self.tcf())
                .field("tfe", &self.tfe())
                .field("tefn", &self.tefn())
                .field("teff", &self.teff())
                .field("tefl", &self.tefl())
                .field("tsw", &self.tsw())
                .field("mraf", &self.mraf())
                .field("too", &self.too())
                .field("elo", &self.elo())
                .field("ep", &self.ep())
                .field("ew", &self.ew())
                .field("bo", &self.bo())
                .field("wdi", &self.wdi())
                .field("pea", &self.pea())
                .field("ped", &self.ped())
                .field("ara", &self.ara())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ir {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ir {{ rfn[0]: {=bool:?}, rfn[1]: {=bool:?}, rff[0]: {=bool:?}, rff[1]: {=bool:?}, rfl[0]: {=bool:?}, rfl[1]: {=bool:?}, hpm: {=bool:?}, tc: {=bool:?}, tcf: {=bool:?}, tfe: {=bool:?}, tefn: {=bool:?}, teff: {=bool:?}, tefl: {=bool:?}, tsw: {=bool:?}, mraf: {=bool:?}, too: {=bool:?}, elo: {=bool:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, wdi: {=bool:?}, pea: {=bool:?}, ped: {=bool:?}, ara: {=bool:?} }}" , self . rfn (0usize) , self . rfn (1usize) , self . rff (0usize) , self . rff (1usize) , self . rfl (0usize) , self . rfl (1usize) , self . hpm () , self . tc () , self . tcf () , self . tfe () , self . tefn () , self . teff () , self . tefl () , self . tsw () , self . mraf () , self . too () , self . elo () , self . ep () , self . ew () , self . bo () , self . wdi () , self . pea () , self . ped () , self . ara ())
        }
    }
    #[doc = "FDCAN nominal bit timing and prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nbtp(pub u32);
    impl Nbtp {
        #[doc = "Nominal time segment after sample point. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used"]
        #[inline(always)]
        pub const fn ntseg2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Nominal time segment after sample point. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used"]
        #[inline(always)]
        pub fn set_ntseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Nominal time segment before sample point. Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn ntseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Nominal time segment before sample point. Valid values are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_ntseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bit rate prescaler. Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn nbrp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bit rate prescaler. Value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_nbrp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "Nominal (re)synchronization jump width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn nsjw(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "Nominal (re)synchronization jump width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that the used value is the one programmed incremented by one. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_nsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for Nbtp {
        #[inline(always)]
        fn default() -> Nbtp {
            Nbtp(0)
        }
    }
    impl core::fmt::Debug for Nbtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nbtp")
                .field("ntseg2", &self.ntseg2())
                .field("ntseg1", &self.ntseg1())
                .field("nbrp", &self.nbrp())
                .field("nsjw", &self.nsjw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nbtp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nbtp {{ ntseg2: {=u8:?}, ntseg1: {=u8:?}, nbrp: {=u16:?}, nsjw: {=u8:?} }}",
                self.ntseg2(),
                self.ntseg1(),
                self.nbrp(),
                self.nsjw()
            )
        }
    }
    #[doc = "FDCAN protocol status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "Last error code. The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
        #[inline(always)]
        pub const fn lec(&self) -> super::vals::Lec {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lec::from_bits(val as u8)
        }
        #[doc = "Last error code. The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
        #[inline(always)]
        pub fn set_lec(&mut self, val: super::vals::Lec) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Activity. Monitors the module’s CAN communication state"]
        #[inline(always)]
        pub const fn act(&self) -> super::vals::Act {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Act::from_bits(val as u8)
        }
        #[doc = "Activity. Monitors the module’s CAN communication state"]
        #[inline(always)]
        pub fn set_act(&mut self, val: super::vals::Act) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Error passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Warning Sstatus"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Sstatus"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Bus_Off status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data last error code. Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
        #[inline(always)]
        pub const fn dlec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Data last error code. Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
        #[inline(always)]
        pub fn set_dlec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "ESI flag of last received FDCAN message. This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
        #[inline(always)]
        pub const fn resi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ESI flag of last received FDCAN message. This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
        #[inline(always)]
        pub fn set_resi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BRS flag of last received FDCAN message. This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
        #[inline(always)]
        pub const fn rbrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BRS flag of last received FDCAN message. This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
        #[inline(always)]
        pub fn set_rbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Received FDCAN message. This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
        #[inline(always)]
        pub const fn redl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Received FDCAN message. This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
        #[inline(always)]
        pub fn set_redl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Protocol exception event"]
        #[inline(always)]
        pub const fn pxe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol exception event"]
        #[inline(always)]
        pub fn set_pxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitter delay compensation value. Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq"]
        #[inline(always)]
        pub const fn tdcv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter delay compensation value. Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq"]
        #[inline(always)]
        pub fn set_tdcv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
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
                .field("lec", &self.lec())
                .field("act", &self.act())
                .field("ep", &self.ep())
                .field("ew", &self.ew())
                .field("bo", &self.bo())
                .field("dlec", &self.dlec())
                .field("resi", &self.resi())
                .field("rbrs", &self.rbrs())
                .field("redl", &self.redl())
                .field("pxe", &self.pxe())
                .field("tdcv", &self.tdcv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Psr {{ lec: {:?}, act: {:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, dlec: {=u8:?}, resi: {=bool:?}, rbrs: {=bool:?}, redl: {=bool:?}, pxe: {=bool:?}, tdcv: {=u8:?} }}" , self . lec () , self . act () , self . ep () , self . ew () , self . bo () , self . dlec () , self . resi () , self . rbrs () , self . redl () , self . pxe () , self . tdcv ())
        }
    }
    #[doc = "FDCAN RAM watchdog register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rwd(pub u32);
    impl Rwd {
        #[doc = "Watchdog configuration. Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of FDCAN_CCCR register are set to 1"]
        #[inline(always)]
        pub const fn wdc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog configuration. Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of FDCAN_CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_wdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Watchdog value. Actual message RAM watchdog counter value"]
        #[inline(always)]
        pub const fn wdv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog value. Actual message RAM watchdog counter value"]
        #[inline(always)]
        pub fn set_wdv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Rwd {
        #[inline(always)]
        fn default() -> Rwd {
            Rwd(0)
        }
    }
    impl core::fmt::Debug for Rwd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rwd")
                .field("wdc", &self.wdc())
                .field("wdv", &self.wdv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rwd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rwd {{ wdc: {=u8:?}, wdv: {=u8:?} }}", self.wdc(), self.wdv())
        }
    }
    #[doc = "CAN Rx FIFO X acknowledge register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfa(pub u32);
    impl Rxfa {
        #[doc = "Rx FIFO X acknowledge index. After the Host has read a message or a sequence of messages from Rx FIFO X it has to write the buffer index of the last element read from Rx FIFO X to FAI. This sets the Rx FIFO X get index RXFS\\[FGI\\]
to FAI + 1 and update the FIFO X fill level RXFS\\[FFL\\]"]
        #[inline(always)]
        pub const fn fai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Rx FIFO X acknowledge index. After the Host has read a message or a sequence of messages from Rx FIFO X it has to write the buffer index of the last element read from Rx FIFO X to FAI. This sets the Rx FIFO X get index RXFS\\[FGI\\]
to FAI + 1 and update the FIFO X fill level RXFS\\[FFL\\]"]
        #[inline(always)]
        pub fn set_fai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Rxfa {
        #[inline(always)]
        fn default() -> Rxfa {
            Rxfa(0)
        }
    }
    impl core::fmt::Debug for Rxfa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxfa").field("fai", &self.fai()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxfa {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxfa {{ fai: {=u8:?} }}", self.fai())
        }
    }
    #[doc = "FDCAN Rx FIFO X status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfs(pub u32);
    impl Rxfs {
        #[doc = "Rx FIFO X fill level. Number of elements stored in Rx FIFO X, range 0 to 3"]
        #[inline(always)]
        pub const fn ffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Rx FIFO X fill level. Number of elements stored in Rx FIFO X, range 0 to 3"]
        #[inline(always)]
        pub fn set_ffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Rx FIFO X get index. Rx FIFO X read index pointer, range 0 to 2"]
        #[inline(always)]
        pub const fn fgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Rx FIFO X get index. Rx FIFO X read index pointer, range 0 to 2"]
        #[inline(always)]
        pub fn set_fgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Rx FIFO X put index. Rx FIFO X write index pointer, range 0 to 2"]
        #[inline(always)]
        pub const fn fpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Rx FIFO X put index. Rx FIFO X write index pointer, range 0 to 2"]
        #[inline(always)]
        pub fn set_fpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Rx FIFO X full"]
        #[inline(always)]
        pub const fn ff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X full"]
        #[inline(always)]
        pub fn set_ff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Rx FIFO X message lost. This bit is a copy of interrupt flag IR\\[RFL\\]. When IR\\[RFL\\]
is reset, this bit is also reset"]
        #[inline(always)]
        pub const fn rfl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X message lost. This bit is a copy of interrupt flag IR\\[RFL\\]. When IR\\[RFL\\]
is reset, this bit is also reset"]
        #[inline(always)]
        pub fn set_rfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Rxfs {
        #[inline(always)]
        fn default() -> Rxfs {
            Rxfs(0)
        }
    }
    impl core::fmt::Debug for Rxfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxfs")
                .field("ffl", &self.ffl())
                .field("fgi", &self.fgi())
                .field("fpi", &self.fpi())
                .field("ff", &self.ff())
                .field("rfl", &self.rfl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxfs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxfs {{ ffl: {=u8:?}, fgi: {=u8:?}, fpi: {=u8:?}, ff: {=bool:?}, rfl: {=bool:?} }}",
                self.ffl(),
                self.fgi(),
                self.fpi(),
                self.ff(),
                self.rfl()
            )
        }
    }
    #[doc = "FDCAN global filter configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxgfc(pub u32);
    impl Rxgfc {
        #[doc = "Reject remote frames extended. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn rrfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reject remote frames extended. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_rrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reject remote frames standard. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn rrfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reject remote frames standard. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_rrfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Accept non-matching frames extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn anfe(&self) -> super::vals::Anfe {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Anfe::from_bits(val as u8)
        }
        #[doc = "Accept non-matching frames extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_anfe(&mut self, val: super::vals::Anfe) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Accept Non-matching frames standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn anfs(&self) -> super::vals::Anfs {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Anfs::from_bits(val as u8)
        }
        #[doc = "Accept Non-matching frames standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_anfs(&mut self, val: super::vals::Anfs) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "FIFO 1 operation mode (overwrite or blocking). This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn f1om(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 1 operation mode (overwrite or blocking). This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_f1om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FIFO 0 operation mode (overwrite or blocking). This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn f0om(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 operation mode (overwrite or blocking). This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_f0om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "List size standard. >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
        #[inline(always)]
        pub const fn lss(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "List size standard. >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
        #[inline(always)]
        pub fn set_lss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "List size extended. >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
        #[inline(always)]
        pub const fn lse(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "List size extended. >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
        #[inline(always)]
        pub fn set_lse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Rxgfc {
        #[inline(always)]
        fn default() -> Rxgfc {
            Rxgfc(0)
        }
    }
    impl core::fmt::Debug for Rxgfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxgfc")
                .field("rrfe", &self.rrfe())
                .field("rrfs", &self.rrfs())
                .field("anfe", &self.anfe())
                .field("anfs", &self.anfs())
                .field("f1om", &self.f1om())
                .field("f0om", &self.f0om())
                .field("lss", &self.lss())
                .field("lse", &self.lse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxgfc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rxgfc {{ rrfe: {=bool:?}, rrfs: {=bool:?}, anfe: {:?}, anfs: {:?}, f1om: {=bool:?}, f0om: {=bool:?}, lss: {=u8:?}, lse: {=u8:?} }}" , self . rrfe () , self . rrfs () , self . anfe () , self . anfs () , self . f1om () , self . f0om () , self . lss () , self . lse ())
        }
    }
    #[doc = "FDCAN transmitter delay compensation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdcr(pub u32);
    impl Tdcr {
        #[doc = "Transmitter delay compensation filter window length. Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn tdcf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter delay compensation filter window length. Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_tdcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Transmitter delay compensation offset. Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn tdco(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter delay compensation offset. Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_tdco(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for Tdcr {
        #[inline(always)]
        fn default() -> Tdcr {
            Tdcr(0)
        }
    }
    impl core::fmt::Debug for Tdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdcr")
                .field("tdcf", &self.tdcf())
                .field("tdco", &self.tdco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tdcr {{ tdcf: {=u8:?}, tdco: {=u8:?} }}", self.tdcf(), self.tdco())
        }
    }
    #[doc = "FDCAN test register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Test(pub u32);
    impl Test {
        #[doc = "Loop back mode"]
        #[inline(always)]
        pub const fn lbck(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop back mode"]
        #[inline(always)]
        pub fn set_lbck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Control of transmit pin"]
        #[inline(always)]
        pub const fn tx(&self) -> super::vals::Tx {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Tx::from_bits(val as u8)
        }
        #[doc = "Control of transmit pin"]
        #[inline(always)]
        pub fn set_tx(&mut self, val: super::vals::Tx) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Receive pin. Monitors the actual value of pin FDCANx_RX"]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive pin. Monitors the actual value of pin FDCANx_RX"]
        #[inline(always)]
        pub fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Test {
        #[inline(always)]
        fn default() -> Test {
            Test(0)
        }
    }
    impl core::fmt::Debug for Test {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Test")
                .field("lbck", &self.lbck())
                .field("tx", &self.tx())
                .field("rx", &self.rx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Test {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Test {{ lbck: {=bool:?}, tx: {:?}, rx: {=bool:?} }}",
                self.lbck(),
                self.tx(),
                self.rx()
            )
        }
    }
    #[doc = "FDCAN timeout counter configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocc(pub u32);
    impl Tocc {
        #[doc = "Timeout counter enable. This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn etoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout counter enable. This is a protected write (P) bit, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_etoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn tos(&self) -> super::vals::Tos {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Tos::from_bits(val as u8)
        }
        #[doc = "Timeout select. When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\\[TOP\\]
and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\\[TOP\\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_tos(&mut self, val: super::vals::Tos) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Timeout period. Start value of the timeout counter (down-counter). Configures the timeout period"]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout period. Start value of the timeout counter (down-counter). Configures the timeout period"]
        #[inline(always)]
        pub fn set_top(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tocc {
        #[inline(always)]
        fn default() -> Tocc {
            Tocc(0)
        }
    }
    impl core::fmt::Debug for Tocc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tocc")
                .field("etoc", &self.etoc())
                .field("tos", &self.tos())
                .field("top", &self.top())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tocc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tocc {{ etoc: {=bool:?}, tos: {:?}, top: {=u16:?} }}",
                self.etoc(),
                self.tos(),
                self.top()
            )
        }
    }
    #[doc = "FDCAN timeout counter value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocv(pub u32);
    impl Tocv {
        #[doc = "Timeout counter. The timeout counter is decremented in multiples of CAN bit times \\[1 … 16\\]
depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS"]
        #[inline(always)]
        pub const fn toc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout counter. The timeout counter is decremented in multiples of CAN bit times \\[1 … 16\\]
depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the timeout counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS"]
        #[inline(always)]
        pub fn set_toc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tocv {
        #[inline(always)]
        fn default() -> Tocv {
            Tocv(0)
        }
    }
    impl core::fmt::Debug for Tocv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tocv").field("toc", &self.toc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tocv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tocv {{ toc: {=u16:?} }}", self.toc())
        }
    }
    #[doc = "FDCAN timestamp counter configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscc(pub u32);
    impl Tscc {
        #[doc = "Timestamp select. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn tss(&self) -> super::vals::Tss {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Tss::from_bits(val as u8)
        }
        #[doc = "Timestamp select. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_tss(&mut self, val: super::vals::Tss) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Timestamp counter prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1 … 16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10). These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn tcp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp counter prescaler. Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1 … 16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10). These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_tcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Tscc {
        #[inline(always)]
        fn default() -> Tscc {
            Tscc(0)
        }
    }
    impl core::fmt::Debug for Tscc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscc")
                .field("tss", &self.tss())
                .field("tcp", &self.tcp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tscc {{ tss: {:?}, tcp: {=u8:?} }}", self.tss(), self.tcp())
        }
    }
    #[doc = "FDCAN timestamp counter value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscv(pub u32);
    impl Tscv {
        #[doc = "Timestamp counter. The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\\[TSS\\]
= 01, the timestamp counter is incremented in multiples of CAN bit times \\[1 … 16\\]
depending on the configuration of TSCC\\[TCP\\]. A wrap around sets interrupt flag IR\\[TSW\\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact"]
        #[inline(always)]
        pub const fn tsc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp counter. The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\\[TSS\\]
= 01, the timestamp counter is incremented in multiples of CAN bit times \\[1 … 16\\]
depending on the configuration of TSCC\\[TCP\\]. A wrap around sets interrupt flag IR\\[TSW\\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact"]
        #[inline(always)]
        pub fn set_tsc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tscv {
        #[inline(always)]
        fn default() -> Tscv {
            Tscv(0)
        }
    }
    impl core::fmt::Debug for Tscv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscv").field("tsc", &self.tsc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tscv {{ tsc: {=u16:?} }}", self.tsc())
        }
    }
    #[doc = "FDCAN Tx buffer add request register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbar(pub u32);
    impl Txbar {
        #[doc = "Add request. Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed"]
        #[inline(always)]
        pub const fn ar(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Add request. Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed"]
        #[inline(always)]
        pub fn set_ar(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbar {
        #[inline(always)]
        fn default() -> Txbar {
            Txbar(0)
        }
    }
    impl core::fmt::Debug for Txbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbar")
                .field("ar[0]", &self.ar(0usize))
                .field("ar[1]", &self.ar(1usize))
                .field("ar[2]", &self.ar(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbar {{ ar[0]: {=bool:?}, ar[1]: {=bool:?}, ar[2]: {=bool:?} }}",
                self.ar(0usize),
                self.ar(1usize),
                self.ar(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx buffer configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbc(pub u32);
    impl Txbc {
        #[doc = "Tx FIFO/queue mode. This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn tfqm(&self) -> super::vals::Tfqm {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Tfqm::from_bits(val as u8)
        }
        #[doc = "Tx FIFO/queue mode. This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_tfqm(&mut self, val: super::vals::Tfqm) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Txbc {
        #[inline(always)]
        fn default() -> Txbc {
            Txbc(0)
        }
    }
    impl core::fmt::Debug for Txbc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbc").field("tfqm", &self.tfqm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbc {{ tfqm: {:?} }}", self.tfqm())
        }
    }
    #[doc = "FDCAN Tx buffer cancellation finished register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcf(pub u32);
    impl Txbcf {
        #[doc = "Cancellation finished. Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR"]
        #[inline(always)]
        pub const fn cf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation finished. Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR"]
        #[inline(always)]
        pub fn set_cf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbcf {
        #[inline(always)]
        fn default() -> Txbcf {
            Txbcf(0)
        }
    }
    impl core::fmt::Debug for Txbcf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbcf")
                .field("cf[0]", &self.cf(0usize))
                .field("cf[1]", &self.cf(1usize))
                .field("cf[2]", &self.cf(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbcf {{ cf[0]: {=bool:?}, cf[1]: {=bool:?}, cf[2]: {=bool:?} }}",
                self.cf(0usize),
                self.cf(1usize),
                self.cf(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcie(pub u32);
    impl Txbcie {
        #[doc = "Cancellation finished interrupt enable.. Each Tx buffer has its own CFIE bit"]
        #[inline(always)]
        pub const fn cfie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation finished interrupt enable.. Each Tx buffer has its own CFIE bit"]
        #[inline(always)]
        pub fn set_cfie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbcie {
        #[inline(always)]
        fn default() -> Txbcie {
            Txbcie(0)
        }
    }
    impl core::fmt::Debug for Txbcie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbcie")
                .field("cfie[0]", &self.cfie(0usize))
                .field("cfie[1]", &self.cfie(1usize))
                .field("cfie[2]", &self.cfie(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbcie {{ cfie[0]: {=bool:?}, cfie[1]: {=bool:?}, cfie[2]: {=bool:?} }}",
                self.cfie(0usize),
                self.cfie(1usize),
                self.cfie(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx buffer cancellation request register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcr(pub u32);
    impl Txbcr {
        #[doc = "Cancellation request. Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset"]
        #[inline(always)]
        pub const fn cr(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation request. Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset"]
        #[inline(always)]
        pub fn set_cr(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbcr {
        #[inline(always)]
        fn default() -> Txbcr {
            Txbcr(0)
        }
    }
    impl core::fmt::Debug for Txbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbcr")
                .field("cr[0]", &self.cr(0usize))
                .field("cr[1]", &self.cr(1usize))
                .field("cr[2]", &self.cr(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbcr {{ cr[0]: {=bool:?}, cr[1]: {=bool:?}, cr[2]: {=bool:?} }}",
                self.cr(0usize),
                self.cr(1usize),
                self.cr(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx buffer request pending register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbrp(pub u32);
    impl Txbrp {
        #[doc = "Transmission request pending. Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions"]
        #[inline(always)]
        pub const fn trp(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission request pending. Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions"]
        #[inline(always)]
        pub fn set_trp(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbrp {
        #[inline(always)]
        fn default() -> Txbrp {
            Txbrp(0)
        }
    }
    impl core::fmt::Debug for Txbrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbrp")
                .field("trp[0]", &self.trp(0usize))
                .field("trp[1]", &self.trp(1usize))
                .field("trp[2]", &self.trp(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbrp {{ trp[0]: {=bool:?}, trp[1]: {=bool:?}, trp[2]: {=bool:?} }}",
                self.trp(0usize),
                self.trp(1usize),
                self.trp(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx buffer transmission interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbtie(pub u32);
    impl Txbtie {
        #[doc = "Transmission interrupt enable. Each Tx buffer has its own TIE bit"]
        #[inline(always)]
        pub const fn tie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission interrupt enable. Each Tx buffer has its own TIE bit"]
        #[inline(always)]
        pub fn set_tie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbtie {
        #[inline(always)]
        fn default() -> Txbtie {
            Txbtie(0)
        }
    }
    impl core::fmt::Debug for Txbtie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbtie")
                .field("tie[0]", &self.tie(0usize))
                .field("tie[1]", &self.tie(1usize))
                .field("tie[2]", &self.tie(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbtie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbtie {{ tie[0]: {=bool:?}, tie[1]: {=bool:?}, tie[2]: {=bool:?} }}",
                self.tie(0usize),
                self.tie(1usize),
                self.tie(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx buffer transmission occurred register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbto(pub u32);
    impl Txbto {
        #[doc = "Transmission occurred.. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR"]
        #[inline(always)]
        pub const fn to(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission occurred.. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR"]
        #[inline(always)]
        pub fn set_to(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbto {
        #[inline(always)]
        fn default() -> Txbto {
            Txbto(0)
        }
    }
    impl core::fmt::Debug for Txbto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbto")
                .field("to[0]", &self.to(0usize))
                .field("to[1]", &self.to(1usize))
                .field("to[2]", &self.to(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbto {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbto {{ to[0]: {=bool:?}, to[1]: {=bool:?}, to[2]: {=bool:?} }}",
                self.to(0usize),
                self.to(1usize),
                self.to(2usize)
            )
        }
    }
    #[doc = "FDCAN Tx event FIFO acknowledge register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefa(pub u32);
    impl Txefa {
        #[doc = "Event FIFO acknowledge index. After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\]
to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]"]
        #[inline(always)]
        pub const fn efai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Event FIFO acknowledge index. After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\]
to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]"]
        #[inline(always)]
        pub fn set_efai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Txefa {
        #[inline(always)]
        fn default() -> Txefa {
            Txefa(0)
        }
    }
    impl core::fmt::Debug for Txefa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txefa").field("efai", &self.efai()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txefa {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txefa {{ efai: {=u8:?} }}", self.efai())
        }
    }
    #[doc = "FDCAN Tx event FIFO status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefs(pub u32);
    impl Txefs {
        #[doc = "Event FIFO fill level. Number of elements stored in Tx event FIFO, range 0 to 3"]
        #[inline(always)]
        pub const fn effl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Event FIFO fill level. Number of elements stored in Tx event FIFO, range 0 to 3"]
        #[inline(always)]
        pub fn set_effl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Event FIFO get index. Tx event FIFO read index pointer, range 0 to 3"]
        #[inline(always)]
        pub const fn efgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Event FIFO get index. Tx event FIFO read index pointer, range 0 to 3"]
        #[inline(always)]
        pub fn set_efgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Event FIFO put index. Tx event FIFO write index pointer, range 0 to 3"]
        #[inline(always)]
        pub const fn efpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Event FIFO put index. Tx event FIFO write index pointer, range 0 to 3"]
        #[inline(always)]
        pub fn set_efpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Event FIFO full"]
        #[inline(always)]
        pub const fn eff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Event FIFO full"]
        #[inline(always)]
        pub fn set_eff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Tx event FIFO element lost. This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\]
is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO element lost. This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\]
is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Txefs {
        #[inline(always)]
        fn default() -> Txefs {
            Txefs(0)
        }
    }
    impl core::fmt::Debug for Txefs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txefs")
                .field("effl", &self.effl())
                .field("efgi", &self.efgi())
                .field("efpi", &self.efpi())
                .field("eff", &self.eff())
                .field("tefl", &self.tefl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txefs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txefs {{ effl: {=u8:?}, efgi: {=u8:?}, efpi: {=u8:?}, eff: {=bool:?}, tefl: {=bool:?} }}",
                self.effl(),
                self.efgi(),
                self.efpi(),
                self.eff(),
                self.tefl()
            )
        }
    }
    #[doc = "FDCAN Tx FIFO/queue status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txfqs(pub u32);
    impl Txfqs {
        #[doc = "Tx FIFO free level. Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)"]
        #[inline(always)]
        pub const fn tffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tx FIFO free level. Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\\[TFQM\\]
= 1)"]
        #[inline(always)]
        pub fn set_tffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Tx FIFO get index. Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
        #[inline(always)]
        pub const fn tfgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Tx FIFO get index. Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)"]
        #[inline(always)]
        pub fn set_tfgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Tx FIFO/queue put index. Tx FIFO/queue write index pointer, range 0 to 3"]
        #[inline(always)]
        pub const fn tfqpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Tx FIFO/queue put index. Tx FIFO/queue write index pointer, range 0 to 3"]
        #[inline(always)]
        pub fn set_tfqpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Tx FIFO/queue full"]
        #[inline(always)]
        pub const fn tfqf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO/queue full"]
        #[inline(always)]
        pub fn set_tfqf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Txfqs {
        #[inline(always)]
        fn default() -> Txfqs {
            Txfqs(0)
        }
    }
    impl core::fmt::Debug for Txfqs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txfqs")
                .field("tffl", &self.tffl())
                .field("tfgi", &self.tfgi())
                .field("tfqpi", &self.tfqpi())
                .field("tfqf", &self.tfqf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txfqs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txfqs {{ tffl: {=u8:?}, tfgi: {=u8:?}, tfqpi: {=u8:?}, tfqf: {=bool:?} }}",
                self.tffl(),
                self.tfgi(),
                self.tfqpi(),
                self.tfqf()
            )
        }
    }
    #[doc = "FDCAN extended ID and mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidam(pub u32);
    impl Xidam {
        #[doc = "Extended ID mask. For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub const fn eidm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Extended ID mask. For acceptance filtering of extended frames the Extended ID AND Mask is AND-ed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to 1 the mask is not active. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1"]
        #[inline(always)]
        pub fn set_eidm(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for Xidam {
        #[inline(always)]
        fn default() -> Xidam {
            Xidam(0)
        }
    }
    impl core::fmt::Debug for Xidam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xidam").field("eidm", &self.eidm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xidam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Xidam {{ eidm: {=u32:?} }}", self.eidm())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Act {
        #[doc = "Synchronizing: node is synchronizing on CAN communication."]
        SYNC = 0x0,
        #[doc = "Idle: node is neither receiver nor transmitter."]
        IDLE = 0x01,
        #[doc = "Receiver: node is operating as receiver."]
        RX = 0x02,
        #[doc = "Transmitter: node is operating as transmitter."]
        TX = 0x03,
    }
    impl Act {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Act {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Act {
        #[inline(always)]
        fn from(val: u8) -> Act {
            Act::from_bits(val)
        }
    }
    impl From<Act> for u8 {
        #[inline(always)]
        fn from(val: Act) -> u8 {
            Act::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Anfe {
        #[doc = "Accept in Rx FIFO 0"]
        ACCEPT_FIFO_0 = 0x0,
        #[doc = "Accept in Rx FIFO 1"]
        ACCEPT_FIFO_1 = 0x01,
        #[doc = "Reject"]
        REJECT = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Anfe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Anfe {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Anfe {
        #[inline(always)]
        fn from(val: u8) -> Anfe {
            Anfe::from_bits(val)
        }
    }
    impl From<Anfe> for u8 {
        #[inline(always)]
        fn from(val: Anfe) -> u8 {
            Anfe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Anfs {
        #[doc = "Accept in Rx FIFO 0"]
        ACCEPT_FIFO_0 = 0x0,
        #[doc = "Accept in Rx FIFO 1"]
        ACCEPT_FIFO_1 = 0x01,
        #[doc = "Reject"]
        REJECT = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Anfs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Anfs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Anfs {
        #[inline(always)]
        fn from(val: u8) -> Anfs {
            Anfs::from_bits(val)
        }
    }
    impl From<Anfs> for u8 {
        #[inline(always)]
        fn from(val: Anfs) -> u8 {
            Anfs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lec {
        #[doc = "No Error: No error occurred since LEC has been reset by successful reception or transmission."]
        NO_ERROR = 0x0,
        #[doc = "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
        STUFF = 0x01,
        #[doc = "Form Error: A fixed format part of a received frame has the wrong format."]
        FORM = 0x02,
        #[doc = "AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
        ACK = 0x03,
        #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
        BIT_1 = 0x04,
        #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
        BIT_0 = 0x05,
        #[doc = "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
        CRC = 0x06,
        #[doc = "NoChange: Any read access to the Protocol status register re-initializes the LEC to ‘7’. When the LEC shows the value ‘7’, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
        NO_CHANGE = 0x07,
    }
    impl Lec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lec {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lec {
        #[inline(always)]
        fn from(val: u8) -> Lec {
            Lec::from_bits(val)
        }
    }
    impl From<Lec> for u8 {
        #[inline(always)]
        fn from(val: Lec) -> u8 {
            Lec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msi {
        #[doc = "No FIFO selected"]
        NO_FIFO = 0x0,
        #[doc = "FIFO overrun"]
        OVERRUN = 0x01,
        #[doc = "Message stored in FIFO 0"]
        FIFO_0 = 0x02,
        #[doc = "Message stored in FIFO 1"]
        FIFO_1 = 0x03,
    }
    impl Msi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msi {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msi {
        #[inline(always)]
        fn from(val: u8) -> Msi {
            Msi::from_bits(val)
        }
    }
    impl From<Msi> for u8 {
        #[inline(always)]
        fn from(val: Msi) -> u8 {
            Msi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pdiv {
        #[doc = "Divide by 1"]
        DIV_1 = 0x0,
        #[doc = "Divide by 2"]
        DIV_2 = 0x01,
        #[doc = "Divide by 4"]
        DIV_4 = 0x02,
        #[doc = "Divide by 6"]
        DIV_6 = 0x03,
        #[doc = "Divide by 8"]
        DIV_8 = 0x04,
        #[doc = "Divide by 10"]
        DIV_10 = 0x05,
        #[doc = "Divide by 12"]
        DIV_12 = 0x06,
        #[doc = "Divide by 14"]
        DIV_14 = 0x07,
        #[doc = "Divide by 16"]
        DIV_16 = 0x08,
        #[doc = "Divide by 18"]
        DIV_18 = 0x09,
        #[doc = "Divide by 20"]
        DIV_20 = 0x0a,
        #[doc = "Divide by 22"]
        DIV_22 = 0x0b,
        #[doc = "Divide by 24"]
        DIV_24 = 0x0c,
        #[doc = "Divide by 26"]
        DIV_26 = 0x0d,
        #[doc = "Divide by 28"]
        DIV_28 = 0x0e,
        #[doc = "Divide by 30"]
        DIV_30 = 0x0f,
    }
    impl Pdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pdiv {
        #[inline(always)]
        fn from(val: u8) -> Pdiv {
            Pdiv::from_bits(val)
        }
    }
    impl From<Pdiv> for u8 {
        #[inline(always)]
        fn from(val: Pdiv) -> u8 {
            Pdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tfqm {
        #[doc = "Tx FIFO operation"]
        FIFO = 0x0,
        #[doc = "Tx queue operation"]
        QUEUE = 0x01,
    }
    impl Tfqm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tfqm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tfqm {
        #[inline(always)]
        fn from(val: u8) -> Tfqm {
            Tfqm::from_bits(val)
        }
    }
    impl From<Tfqm> for u8 {
        #[inline(always)]
        fn from(val: Tfqm) -> u8 {
            Tfqm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tos {
        #[doc = "Continuous operation"]
        CONTINUOUS = 0x0,
        #[doc = "Timeout controlled by Tx event FIFO"]
        TX_EVENT_FIFO = 0x01,
        #[doc = "Timeout controlled by Rx FIFO 0"]
        RX_FIFO_0 = 0x02,
        #[doc = "Timeout controlled by Rx FIFO 1"]
        RX_FIFO_1 = 0x03,
    }
    impl Tos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tos {
        #[inline(always)]
        fn from(val: u8) -> Tos {
            Tos::from_bits(val)
        }
    }
    impl From<Tos> for u8 {
        #[inline(always)]
        fn from(val: Tos) -> u8 {
            Tos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tss {
        #[doc = "Timestamp counter value always 0x0000"]
        ZERO = 0x0,
        #[doc = "Timestamp counter value incremented according to TCP"]
        INCREMENT = 0x01,
        #[doc = "External timestamp counter from TIM3 value (tim3_cnt\\[0:15\\])"]
        EXTERNAL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Tss {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tss {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tss {
        #[inline(always)]
        fn from(val: u8) -> Tss {
            Tss::from_bits(val)
        }
    }
    impl From<Tss> for u8 {
        #[inline(always)]
        fn from(val: Tss) -> u8 {
            Tss::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tx {
        #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
        RESET = 0x0,
        #[doc = "Sample point can be monitored at pin FDCANx_TX"]
        SAMPLE_POINT = 0x01,
        #[doc = "Dominant (0) level at pin FDCANx_TX"]
        DOMINANT = 0x02,
        #[doc = "Recessive (1) at pin FDCANx_TX"]
        RECESSIVE = 0x03,
    }
    impl Tx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tx {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tx {
        #[inline(always)]
        fn from(val: u8) -> Tx {
            Tx::from_bits(val)
        }
    }
    impl From<Tx> for u8 {
        #[inline(always)]
        fn from(val: Tx) -> u8 {
            Tx::to_bits(val)
        }
    }
}
