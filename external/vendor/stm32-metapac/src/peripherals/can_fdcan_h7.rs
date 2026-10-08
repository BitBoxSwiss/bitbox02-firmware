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
    #[doc = "FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn crel(self) -> crate::common::Reg<regs::Crel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<regs::Endn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FDCAN Data Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FDCAN Test Register"]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FDCAN RAM Watchdog Register"]
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<regs::Rwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "FDCAN CC Control Register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::Tocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FDCAN Interrupt Register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FDCAN Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FDCAN Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "FDCAN Interrupt Line Enable Register"]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "FDCAN Global Filter Configuration Register"]
    #[inline(always)]
    pub const fn gfc(self) -> crate::common::Reg<regs::Gfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FDCAN Standard ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn sidfc(self) -> crate::common::Reg<regs::Sidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FDCAN Extended ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn xidfc(self) -> crate::common::Reg<regs::Xidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "FDCAN High Priority Message Status Register"]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::Hpms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "FDCAN New Data 1 Register"]
    #[inline(always)]
    pub const fn ndat1(self) -> crate::common::Reg<regs::Ndat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "FDCAN New Data 2 Register"]
    #[inline(always)]
    pub const fn ndat2(self) -> crate::common::Reg<regs::Ndat2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "FDCAN Rx FIFO X Configuration Register"]
    #[inline(always)]
    pub const fn rxfc(self, n: usize) -> crate::common::Reg<regs::Rxfc, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + ([0usize, 16usize][n] as usize)) as _) }
    }
    #[doc = "FDCAN Rx FIFO X Status Register"]
    #[inline(always)]
    pub const fn rxfs(self, n: usize) -> crate::common::Reg<regs::Rxfs, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize + ([0usize, 16usize][n] as usize)) as _) }
    }
    #[doc = "CAN Rx FIFO X Acknowledge Register"]
    #[inline(always)]
    pub const fn rxfa(self, n: usize) -> crate::common::Reg<regs::Rxfa, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize + ([0usize, 16usize][n] as usize)) as _) }
    }
    #[doc = "FDCAN Rx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn rxbc(self) -> crate::common::Reg<regs::Rxbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn rxesc(self) -> crate::common::Reg<regs::Rxesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FDCAN Tx FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::Txfqs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn txesc(self) -> crate::common::Reg<regs::Txesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::Txbrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::Txbto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::Txbcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Configuration Register"]
    #[inline(always)]
    pub const fn txefc(self) -> crate::common::Reg<regs::Txefc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::Txefs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FDCAN TT Trigger Memory Configuration Register"]
    #[inline(always)]
    pub const fn tttmc(self) -> crate::common::Reg<regs::Tttmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "FDCAN TT Reference Message Configuration Register"]
    #[inline(always)]
    pub const fn ttrmc(self) -> crate::common::Reg<regs::Ttrmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "FDCAN TT Operation Configuration Register"]
    #[inline(always)]
    pub const fn ttocf(self) -> crate::common::Reg<regs::Ttocf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "FDCAN TT Matrix Limits Register"]
    #[inline(always)]
    pub const fn ttmlm(self) -> crate::common::Reg<regs::Ttmlm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "FDCAN TUR Configuration Register"]
    #[inline(always)]
    pub const fn turcf(self) -> crate::common::Reg<regs::Turcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "FDCAN TT Operation Control Register"]
    #[inline(always)]
    pub const fn ttocn(self) -> crate::common::Reg<regs::Ttocn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "FDCAN TT Global Time Preset Register"]
    #[inline(always)]
    pub const fn ttgtp(self) -> crate::common::Reg<regs::Ttgtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "FDCAN TT Time Mark Register"]
    #[inline(always)]
    pub const fn tttmk(self) -> crate::common::Reg<regs::Tttmk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "FDCAN TT Interrupt Register"]
    #[inline(always)]
    pub const fn ttir(self) -> crate::common::Reg<regs::Ttir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "FDCAN TT Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ttie(self) -> crate::common::Reg<regs::Ttie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "FDCAN TT Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn ttils(self) -> crate::common::Reg<regs::Ttils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "FDCAN TT Operation Status Register"]
    #[inline(always)]
    pub const fn ttost(self) -> crate::common::Reg<regs::Ttost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "FDCAN TUR Numerator Actual Register"]
    #[inline(always)]
    pub const fn turna(self) -> crate::common::Reg<regs::Turna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "FDCAN TT Local and Global Time Register"]
    #[inline(always)]
    pub const fn ttlgt(self) -> crate::common::Reg<regs::Ttlgt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "FDCAN TT Cycle Time and Count Register"]
    #[inline(always)]
    pub const fn ttctc(self) -> crate::common::Reg<regs::Ttctc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "FDCAN TT Capture Time Register"]
    #[inline(always)]
    pub const fn ttcpt(self) -> crate::common::Reg<regs::Ttcpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "FDCAN TT Cycle Sync Mark Register"]
    #[inline(always)]
    pub const fn ttcsm(self) -> crate::common::Reg<regs::Ttcsm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "FDCAN TT Trigger Select Register"]
    #[inline(always)]
    pub const fn ttts(self) -> crate::common::Reg<regs::Ttts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
}
pub mod regs {
    #[doc = "FDCAN CC Control Register"]
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
        #[doc = "Configuration Change Enable"]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Change Enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ASM Restricted Operation Mode"]
        #[inline(always)]
        pub const fn asm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ASM Restricted Operation Mode"]
        #[inline(always)]
        pub fn set_asm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock Stop Acknowledge"]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Stop Acknowledge"]
        #[inline(always)]
        pub fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock Stop Request"]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Stop Request"]
        #[inline(always)]
        pub fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bus Monitoring Mode"]
        #[inline(always)]
        pub const fn mon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Monitoring Mode"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable Automatic Retransmission"]
        #[inline(always)]
        pub const fn dar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Automatic Retransmission"]
        #[inline(always)]
        pub fn set_dar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Test Mode Enable"]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Test Mode Enable"]
        #[inline(always)]
        pub fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FD Operation Enable"]
        #[inline(always)]
        pub const fn fdoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FD Operation Enable"]
        #[inline(always)]
        pub fn set_fdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FDCAN Bit Rate Switching"]
        #[inline(always)]
        pub const fn bse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Bit Rate Switching"]
        #[inline(always)]
        pub fn set_bse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Protocol Exception Handling Disable"]
        #[inline(always)]
        pub const fn pxhd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Exception Handling Disable"]
        #[inline(always)]
        pub fn set_pxhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Edge Filtering during Bus Integration"]
        #[inline(always)]
        pub const fn efbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Edge Filtering during Bus Integration"]
        #[inline(always)]
        pub fn set_efbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TXP"]
        #[inline(always)]
        pub const fn txp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TXP"]
        #[inline(always)]
        pub fn set_txp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Non ISO Operation"]
        #[inline(always)]
        pub const fn niso(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Non ISO Operation"]
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
                .field("bse", &self.bse())
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
            defmt :: write ! (f , "Cccr {{ init: {=bool:?}, cce: {=bool:?}, asm: {=bool:?}, csa: {=bool:?}, csr: {=bool:?}, mon: {=bool:?}, dar: {=bool:?}, test: {=bool:?}, fdoe: {=bool:?}, bse: {=bool:?}, pxhd: {=bool:?}, efbi: {=bool:?}, txp: {=bool:?}, niso: {=bool:?} }}" , self . init () , self . cce () , self . asm () , self . csa () , self . csr () , self . mon () , self . dar () , self . test () , self . fdoe () , self . bse () , self . pxhd () , self . efbi () , self . txp () , self . niso ())
        }
    }
    #[doc = "FDCAN Core Release Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crel(pub u32);
    impl Crel {
        #[doc = "Timestamp Day"]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Timestamp Day"]
        #[inline(always)]
        pub fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Timestamp Month"]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timestamp Month"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Timestamp Year"]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Year"]
        #[inline(always)]
        pub fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Sub-step of Core release"]
        #[inline(always)]
        pub const fn substep(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Sub-step of Core release"]
        #[inline(always)]
        pub fn set_substep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Step of Core release"]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Step of Core release"]
        #[inline(always)]
        pub fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Core release"]
        #[inline(always)]
        pub const fn rel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Core release"]
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
    #[doc = "FDCAN Data Bit Timing and Prescaler Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbtp(pub u32);
    impl Dbtp {
        #[doc = "Synchronization Jump Width"]
        #[inline(always)]
        pub const fn dsjw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Synchronization Jump Width"]
        #[inline(always)]
        pub fn set_dsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub const fn dtseg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub fn set_dtseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub const fn dtseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub fn set_dtseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data BIt Rate Prescaler"]
        #[inline(always)]
        pub const fn dbrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Data BIt Rate Prescaler"]
        #[inline(always)]
        pub fn set_dbrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Transceiver Delay Compensation"]
        #[inline(always)]
        pub const fn tdc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transceiver Delay Compensation"]
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
    #[doc = "FDCAN Error Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "Transmit Error Counter"]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit Error Counter"]
        #[inline(always)]
        pub fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Receive Error Counter"]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Receive Error Counter"]
        #[inline(always)]
        pub fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Receive Error Passive"]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Error Passive"]
        #[inline(always)]
        pub fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AN Error Logging"]
        #[inline(always)]
        pub const fn cel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "AN Error Logging"]
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
    #[doc = "FDCAN Core Release Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Endn(pub u32);
    impl Endn {
        #[doc = "Endiannes Test Value"]
        #[inline(always)]
        pub const fn etv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Endiannes Test Value"]
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
    #[doc = "FDCAN Global Filter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gfc(pub u32);
    impl Gfc {
        #[doc = "Reject Remote Frames Extended"]
        #[inline(always)]
        pub const fn rrfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reject Remote Frames Extended"]
        #[inline(always)]
        pub fn set_rrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reject Remote Frames Standard"]
        #[inline(always)]
        pub const fn rrfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reject Remote Frames Standard"]
        #[inline(always)]
        pub fn set_rrfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Accept Non-matching Frames Extended"]
        #[inline(always)]
        pub const fn anfe(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Accept Non-matching Frames Extended"]
        #[inline(always)]
        pub fn set_anfe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Accept Non-matching Frames Standard"]
        #[inline(always)]
        pub const fn anfs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Accept Non-matching Frames Standard"]
        #[inline(always)]
        pub fn set_anfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Gfc {
        #[inline(always)]
        fn default() -> Gfc {
            Gfc(0)
        }
    }
    impl core::fmt::Debug for Gfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gfc")
                .field("rrfe", &self.rrfe())
                .field("rrfs", &self.rrfs())
                .field("anfe", &self.anfe())
                .field("anfs", &self.anfs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gfc {{ rrfe: {=bool:?}, rrfs: {=bool:?}, anfe: {=u8:?}, anfs: {=u8:?} }}",
                self.rrfe(),
                self.rrfs(),
                self.anfe(),
                self.anfs()
            )
        }
    }
    #[doc = "FDCAN High Priority Message Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hpms(pub u32);
    impl Hpms {
        #[doc = "Buffer Index"]
        #[inline(always)]
        pub const fn bidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Buffer Index"]
        #[inline(always)]
        pub fn set_bidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Message Storage Indicator"]
        #[inline(always)]
        pub const fn msi(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Message Storage Indicator"]
        #[inline(always)]
        pub fn set_msi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Filter Index"]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Filter Index"]
        #[inline(always)]
        pub fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Filter List"]
        #[inline(always)]
        pub const fn flst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter List"]
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
                "Hpms {{ bidx: {=u8:?}, msi: {=u8:?}, fidx: {=u8:?}, flst: {=bool:?} }}",
                self.bidx(),
                self.msi(),
                self.fidx(),
                self.flst()
            )
        }
    }
    #[doc = "FDCAN Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ie(pub u32);
    impl Ie {
        #[doc = "Rx FIFO X New Message Enable"]
        #[inline(always)]
        pub const fn rfne(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X New Message Enable"]
        #[inline(always)]
        pub fn set_rfne(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Watermark Reached Enable"]
        #[inline(always)]
        pub const fn rfwe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Watermark Reached Enable"]
        #[inline(always)]
        pub fn set_rfwe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Full Enable"]
        #[inline(always)]
        pub const fn rffe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full Enable"]
        #[inline(always)]
        pub fn set_rffe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Message Lost Enable"]
        #[inline(always)]
        pub const fn rfle(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost Enable"]
        #[inline(always)]
        pub fn set_rfle(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High Priority Message Enable"]
        #[inline(always)]
        pub const fn hpme(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message Enable"]
        #[inline(always)]
        pub fn set_hpme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed Enable"]
        #[inline(always)]
        pub const fn tce(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed Enable"]
        #[inline(always)]
        pub fn set_tce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished Enable"]
        #[inline(always)]
        pub const fn tcfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished Enable"]
        #[inline(always)]
        pub fn set_tcfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty Enable"]
        #[inline(always)]
        pub const fn tefe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty Enable"]
        #[inline(always)]
        pub fn set_tefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry Enable"]
        #[inline(always)]
        pub const fn tefne(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry Enable"]
        #[inline(always)]
        pub fn set_tefne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached Enable"]
        #[inline(always)]
        pub const fn tefwe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached Enable"]
        #[inline(always)]
        pub fn set_tefwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full Enable"]
        #[inline(always)]
        pub const fn teffe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full Enable"]
        #[inline(always)]
        pub fn set_teffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost Enable"]
        #[inline(always)]
        pub const fn tefle(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost Enable"]
        #[inline(always)]
        pub fn set_tefle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound Enable"]
        #[inline(always)]
        pub const fn tswe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound Enable"]
        #[inline(always)]
        pub fn set_tswe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure Enable"]
        #[inline(always)]
        pub const fn mrafe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure Enable"]
        #[inline(always)]
        pub fn set_mrafe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred Enable"]
        #[inline(always)]
        pub const fn tooe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred Enable"]
        #[inline(always)]
        pub fn set_tooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer Enable"]
        #[inline(always)]
        pub const fn drxe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer Enable"]
        #[inline(always)]
        pub fn set_drxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Interrupt Enable"]
        #[inline(always)]
        pub const fn bece(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Interrupt Enable"]
        #[inline(always)]
        pub fn set_bece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Interrupt Enable"]
        #[inline(always)]
        pub const fn beue(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Interrupt Enable"]
        #[inline(always)]
        pub fn set_beue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow Enable"]
        #[inline(always)]
        pub const fn eloe(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow Enable"]
        #[inline(always)]
        pub fn set_eloe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive Enable"]
        #[inline(always)]
        pub const fn epe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Enable"]
        #[inline(always)]
        pub fn set_epe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status Enable"]
        #[inline(always)]
        pub const fn ewe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status Enable"]
        #[inline(always)]
        pub fn set_ewe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status Enable"]
        #[inline(always)]
        pub const fn boe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status Enable"]
        #[inline(always)]
        pub fn set_boe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt Enable"]
        #[inline(always)]
        pub const fn wdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt Enable"]
        #[inline(always)]
        pub fn set_wdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase Enable"]
        #[inline(always)]
        pub const fn peae(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase Enable"]
        #[inline(always)]
        pub fn set_peae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase Enable"]
        #[inline(always)]
        pub const fn pede(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase Enable"]
        #[inline(always)]
        pub fn set_pede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address Enable"]
        #[inline(always)]
        pub const fn arae(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address Enable"]
        #[inline(always)]
        pub fn set_arae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("rfwe[0]", &self.rfwe(0usize))
                .field("rfwe[1]", &self.rfwe(1usize))
                .field("rffe[0]", &self.rffe(0usize))
                .field("rffe[1]", &self.rffe(1usize))
                .field("rfle[0]", &self.rfle(0usize))
                .field("rfle[1]", &self.rfle(1usize))
                .field("hpme", &self.hpme())
                .field("tce", &self.tce())
                .field("tcfe", &self.tcfe())
                .field("tefe", &self.tefe())
                .field("tefne", &self.tefne())
                .field("tefwe", &self.tefwe())
                .field("teffe", &self.teffe())
                .field("tefle", &self.tefle())
                .field("tswe", &self.tswe())
                .field("mrafe", &self.mrafe())
                .field("tooe", &self.tooe())
                .field("drxe", &self.drxe())
                .field("bece", &self.bece())
                .field("beue", &self.beue())
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
            defmt :: write ! (f , "Ie {{ rfne[0]: {=bool:?}, rfne[1]: {=bool:?}, rfwe[0]: {=bool:?}, rfwe[1]: {=bool:?}, rffe[0]: {=bool:?}, rffe[1]: {=bool:?}, rfle[0]: {=bool:?}, rfle[1]: {=bool:?}, hpme: {=bool:?}, tce: {=bool:?}, tcfe: {=bool:?}, tefe: {=bool:?}, tefne: {=bool:?}, tefwe: {=bool:?}, teffe: {=bool:?}, tefle: {=bool:?}, tswe: {=bool:?}, mrafe: {=bool:?}, tooe: {=bool:?}, drxe: {=bool:?}, bece: {=bool:?}, beue: {=bool:?}, eloe: {=bool:?}, epe: {=bool:?}, ewe: {=bool:?}, boe: {=bool:?}, wdie: {=bool:?}, peae: {=bool:?}, pede: {=bool:?}, arae: {=bool:?} }}" , self . rfne (0usize) , self . rfne (1usize) , self . rfwe (0usize) , self . rfwe (1usize) , self . rffe (0usize) , self . rffe (1usize) , self . rfle (0usize) , self . rfle (1usize) , self . hpme () , self . tce () , self . tcfe () , self . tefe () , self . tefne () , self . tefwe () , self . teffe () , self . tefle () , self . tswe () , self . mrafe () , self . tooe () , self . drxe () , self . bece () , self . beue () , self . eloe () , self . epe () , self . ewe () , self . boe () , self . wdie () , self . peae () , self . pede () , self . arae ())
        }
    }
    #[doc = "FDCAN Interrupt Line Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ile(pub u32);
    impl Ile {
        #[doc = "Enable Interrupt Line 0"]
        #[inline(always)]
        pub const fn eint0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt Line 0"]
        #[inline(always)]
        pub fn set_eint0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Interrupt Line 1"]
        #[inline(always)]
        pub const fn eint1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt Line 1"]
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
    #[doc = "FDCAN Interrupt Line Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ils(pub u32);
    impl Ils {
        #[doc = "Rx FIFO X New Message Interrupt Line"]
        #[inline(always)]
        pub const fn rfnl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X New Message Interrupt Line"]
        #[inline(always)]
        pub fn set_rfnl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub const fn rfwl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub fn set_rfwl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Full Interrupt Line"]
        #[inline(always)]
        pub const fn rffl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full Interrupt Line"]
        #[inline(always)]
        pub fn set_rffl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Message Lost Interrupt Line"]
        #[inline(always)]
        pub const fn rfll(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost Interrupt Line"]
        #[inline(always)]
        pub fn set_rfll(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High Priority Message Interrupt Line"]
        #[inline(always)]
        pub const fn hpml(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message Interrupt Line"]
        #[inline(always)]
        pub fn set_hpml(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed Interrupt Line"]
        #[inline(always)]
        pub const fn tcl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed Interrupt Line"]
        #[inline(always)]
        pub fn set_tcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line"]
        #[inline(always)]
        pub const fn tcfl(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line"]
        #[inline(always)]
        pub fn set_tcfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty Interrupt Line"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty Interrupt Line"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line"]
        #[inline(always)]
        pub const fn tefnl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line"]
        #[inline(always)]
        pub fn set_tefnl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub const fn tefwl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub fn set_tefwl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full Interrupt Line"]
        #[inline(always)]
        pub const fn teffl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full Interrupt Line"]
        #[inline(always)]
        pub fn set_teffl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Line"]
        #[inline(always)]
        pub const fn tefll(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Line"]
        #[inline(always)]
        pub fn set_tefll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound Interrupt Line"]
        #[inline(always)]
        pub const fn tswl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound Interrupt Line"]
        #[inline(always)]
        pub fn set_tswl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure Interrupt Line"]
        #[inline(always)]
        pub const fn mrafl(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure Interrupt Line"]
        #[inline(always)]
        pub fn set_mrafl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred Interrupt Line"]
        #[inline(always)]
        pub const fn tool(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred Interrupt Line"]
        #[inline(always)]
        pub fn set_tool(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line"]
        #[inline(always)]
        pub const fn drxl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line"]
        #[inline(always)]
        pub fn set_drxl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Interrupt Line"]
        #[inline(always)]
        pub const fn becl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Interrupt Line"]
        #[inline(always)]
        pub fn set_becl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Interrupt Line"]
        #[inline(always)]
        pub const fn beul(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Interrupt Line"]
        #[inline(always)]
        pub fn set_beul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow Interrupt Line"]
        #[inline(always)]
        pub const fn elol(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow Interrupt Line"]
        #[inline(always)]
        pub fn set_elol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive Interrupt Line"]
        #[inline(always)]
        pub const fn epl(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Interrupt Line"]
        #[inline(always)]
        pub fn set_epl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status Interrupt Line"]
        #[inline(always)]
        pub const fn ewl(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status Interrupt Line"]
        #[inline(always)]
        pub fn set_ewl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub const fn bol(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub fn set_bol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt Line"]
        #[inline(always)]
        pub const fn wdil(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt Line"]
        #[inline(always)]
        pub fn set_wdil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase Line"]
        #[inline(always)]
        pub const fn peal(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase Line"]
        #[inline(always)]
        pub fn set_peal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase Line"]
        #[inline(always)]
        pub const fn pedl(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase Line"]
        #[inline(always)]
        pub fn set_pedl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address Line"]
        #[inline(always)]
        pub const fn aral(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address Line"]
        #[inline(always)]
        pub fn set_aral(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("rfnl[0]", &self.rfnl(0usize))
                .field("rfnl[1]", &self.rfnl(1usize))
                .field("rfwl[0]", &self.rfwl(0usize))
                .field("rfwl[1]", &self.rfwl(1usize))
                .field("rffl[0]", &self.rffl(0usize))
                .field("rffl[1]", &self.rffl(1usize))
                .field("rfll[0]", &self.rfll(0usize))
                .field("rfll[1]", &self.rfll(1usize))
                .field("hpml", &self.hpml())
                .field("tcl", &self.tcl())
                .field("tcfl", &self.tcfl())
                .field("tefl", &self.tefl())
                .field("tefnl", &self.tefnl())
                .field("tefwl", &self.tefwl())
                .field("teffl", &self.teffl())
                .field("tefll", &self.tefll())
                .field("tswl", &self.tswl())
                .field("mrafl", &self.mrafl())
                .field("tool", &self.tool())
                .field("drxl", &self.drxl())
                .field("becl", &self.becl())
                .field("beul", &self.beul())
                .field("elol", &self.elol())
                .field("epl", &self.epl())
                .field("ewl", &self.ewl())
                .field("bol", &self.bol())
                .field("wdil", &self.wdil())
                .field("peal", &self.peal())
                .field("pedl", &self.pedl())
                .field("aral", &self.aral())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ils {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ils {{ rfnl[0]: {=bool:?}, rfnl[1]: {=bool:?}, rfwl[0]: {=bool:?}, rfwl[1]: {=bool:?}, rffl[0]: {=bool:?}, rffl[1]: {=bool:?}, rfll[0]: {=bool:?}, rfll[1]: {=bool:?}, hpml: {=bool:?}, tcl: {=bool:?}, tcfl: {=bool:?}, tefl: {=bool:?}, tefnl: {=bool:?}, tefwl: {=bool:?}, teffl: {=bool:?}, tefll: {=bool:?}, tswl: {=bool:?}, mrafl: {=bool:?}, tool: {=bool:?}, drxl: {=bool:?}, becl: {=bool:?}, beul: {=bool:?}, elol: {=bool:?}, epl: {=bool:?}, ewl: {=bool:?}, bol: {=bool:?}, wdil: {=bool:?}, peal: {=bool:?}, pedl: {=bool:?}, aral: {=bool:?} }}" , self . rfnl (0usize) , self . rfnl (1usize) , self . rfwl (0usize) , self . rfwl (1usize) , self . rffl (0usize) , self . rffl (1usize) , self . rfll (0usize) , self . rfll (1usize) , self . hpml () , self . tcl () , self . tcfl () , self . tefl () , self . tefnl () , self . tefwl () , self . teffl () , self . tefll () , self . tswl () , self . mrafl () , self . tool () , self . drxl () , self . becl () , self . beul () , self . elol () , self . epl () , self . ewl () , self . bol () , self . wdil () , self . peal () , self . pedl () , self . aral ())
        }
    }
    #[doc = "FDCAN Interrupt Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Rx FIFO X New Message"]
        #[inline(always)]
        pub const fn rfn(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X New Message"]
        #[inline(always)]
        pub fn set_rfn(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Watermark Reached"]
        #[inline(always)]
        pub const fn rfw(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Watermark Reached"]
        #[inline(always)]
        pub fn set_rfw(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub const fn rff(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub fn set_rff(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub const fn rfl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub fn set_rfl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High Priority Message"]
        #[inline(always)]
        pub const fn hpm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message"]
        #[inline(always)]
        pub fn set_hpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished"]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished"]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty"]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty"]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry"]
        #[inline(always)]
        pub const fn tefn(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry"]
        #[inline(always)]
        pub fn set_tefn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached"]
        #[inline(always)]
        pub const fn tefw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached"]
        #[inline(always)]
        pub fn set_tefw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full"]
        #[inline(always)]
        pub const fn teff(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full"]
        #[inline(always)]
        pub fn set_teff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound"]
        #[inline(always)]
        pub const fn tsw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound"]
        #[inline(always)]
        pub fn set_tsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure"]
        #[inline(always)]
        pub const fn mraf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure"]
        #[inline(always)]
        pub fn set_mraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred"]
        #[inline(always)]
        pub const fn too(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred"]
        #[inline(always)]
        pub fn set_too(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer"]
        #[inline(always)]
        pub const fn drx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer"]
        #[inline(always)]
        pub fn set_drx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Error Logging Overflow"]
        #[inline(always)]
        pub const fn elo(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow"]
        #[inline(always)]
        pub fn set_elo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt"]
        #[inline(always)]
        pub const fn wdi(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt"]
        #[inline(always)]
        pub fn set_wdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
        #[inline(always)]
        pub const fn pea(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
        #[inline(always)]
        pub fn set_pea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase (Data Bit Time is used)"]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase (Data Bit Time is used)"]
        #[inline(always)]
        pub fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address"]
        #[inline(always)]
        pub const fn ara(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address"]
        #[inline(always)]
        pub fn set_ara(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("rfw[0]", &self.rfw(0usize))
                .field("rfw[1]", &self.rfw(1usize))
                .field("rff[0]", &self.rff(0usize))
                .field("rff[1]", &self.rff(1usize))
                .field("rfl[0]", &self.rfl(0usize))
                .field("rfl[1]", &self.rfl(1usize))
                .field("hpm", &self.hpm())
                .field("tc", &self.tc())
                .field("tcf", &self.tcf())
                .field("tef", &self.tef())
                .field("tefn", &self.tefn())
                .field("tefw", &self.tefw())
                .field("teff", &self.teff())
                .field("tefl", &self.tefl())
                .field("tsw", &self.tsw())
                .field("mraf", &self.mraf())
                .field("too", &self.too())
                .field("drx", &self.drx())
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
            defmt :: write ! (f , "Ir {{ rfn[0]: {=bool:?}, rfn[1]: {=bool:?}, rfw[0]: {=bool:?}, rfw[1]: {=bool:?}, rff[0]: {=bool:?}, rff[1]: {=bool:?}, rfl[0]: {=bool:?}, rfl[1]: {=bool:?}, hpm: {=bool:?}, tc: {=bool:?}, tcf: {=bool:?}, tef: {=bool:?}, tefn: {=bool:?}, tefw: {=bool:?}, teff: {=bool:?}, tefl: {=bool:?}, tsw: {=bool:?}, mraf: {=bool:?}, too: {=bool:?}, drx: {=bool:?}, elo: {=bool:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, wdi: {=bool:?}, pea: {=bool:?}, ped: {=bool:?}, ara: {=bool:?} }}" , self . rfn (0usize) , self . rfn (1usize) , self . rfw (0usize) , self . rfw (1usize) , self . rff (0usize) , self . rff (1usize) , self . rfl (0usize) , self . rfl (1usize) , self . hpm () , self . tc () , self . tcf () , self . tef () , self . tefn () , self . tefw () , self . teff () , self . tefl () , self . tsw () , self . mraf () , self . too () , self . drx () , self . elo () , self . ep () , self . ew () , self . bo () , self . wdi () , self . pea () , self . ped () , self . ara ())
        }
    }
    #[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nbtp(pub u32);
    impl Nbtp {
        #[doc = "Nominal Time segment after sample point"]
        #[inline(always)]
        pub const fn ntseg2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Nominal Time segment after sample point"]
        #[inline(always)]
        pub fn set_ntseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Nominal Time segment before sample point"]
        #[inline(always)]
        pub const fn ntseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Nominal Time segment before sample point"]
        #[inline(always)]
        pub fn set_ntseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bit Rate Prescaler"]
        #[inline(always)]
        pub const fn nbrp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bit Rate Prescaler"]
        #[inline(always)]
        pub fn set_nbrp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "NSJW: Nominal (Re)Synchronization Jump Width."]
        #[inline(always)]
        pub const fn nsjw(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "NSJW: Nominal (Re)Synchronization Jump Width."]
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
    #[doc = "FDCAN New Data 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat1(pub u32);
    impl Ndat1 {
        #[doc = "New data (buffers 0 - 31)"]
        #[inline(always)]
        pub const fn nd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New data (buffers 0 - 31)"]
        #[inline(always)]
        pub fn set_nd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ndat1 {
        #[inline(always)]
        fn default() -> Ndat1 {
            Ndat1(0)
        }
    }
    impl core::fmt::Debug for Ndat1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ndat1").field("nd", &self.nd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ndat1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ndat1 {{ nd: {=u32:?} }}", self.nd())
        }
    }
    #[doc = "FDCAN New Data 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat2(pub u32);
    impl Ndat2 {
        #[doc = "New data (buffers 32 - 63)"]
        #[inline(always)]
        pub const fn nd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New data (buffers 32 - 63)"]
        #[inline(always)]
        pub fn set_nd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ndat2 {
        #[inline(always)]
        fn default() -> Ndat2 {
            Ndat2(0)
        }
    }
    impl core::fmt::Debug for Ndat2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ndat2").field("nd", &self.nd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ndat2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ndat2 {{ nd: {=u32:?} }}", self.nd())
        }
    }
    #[doc = "FDCAN Protocol Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "Last Error Code"]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Last Error Code"]
        #[inline(always)]
        pub fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Activity"]
        #[inline(always)]
        pub const fn act(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Activity"]
        #[inline(always)]
        pub fn set_act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data Last Error Code"]
        #[inline(always)]
        pub const fn dlec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Data Last Error Code"]
        #[inline(always)]
        pub fn set_dlec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "ESI flag of last received FDCAN Message"]
        #[inline(always)]
        pub const fn resi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ESI flag of last received FDCAN Message"]
        #[inline(always)]
        pub fn set_resi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BRS flag of last received FDCAN Message"]
        #[inline(always)]
        pub const fn rbrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BRS flag of last received FDCAN Message"]
        #[inline(always)]
        pub fn set_rbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Received FDCAN Message"]
        #[inline(always)]
        pub const fn redl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Received FDCAN Message"]
        #[inline(always)]
        pub fn set_redl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Protocol Exception Event"]
        #[inline(always)]
        pub const fn pxe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Exception Event"]
        #[inline(always)]
        pub fn set_pxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitter Delay Compensation Value"]
        #[inline(always)]
        pub const fn tdcv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Value"]
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
            defmt :: write ! (f , "Psr {{ lec: {=u8:?}, act: {=u8:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, dlec: {=u8:?}, resi: {=bool:?}, rbrs: {=bool:?}, redl: {=bool:?}, pxe: {=bool:?}, tdcv: {=u8:?} }}" , self . lec () , self . act () , self . ep () , self . ew () , self . bo () , self . dlec () , self . resi () , self . rbrs () , self . redl () , self . pxe () , self . tdcv ())
        }
    }
    #[doc = "FDCAN RAM Watchdog Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rwd(pub u32);
    impl Rwd {
        #[doc = "Watchdog configuration"]
        #[inline(always)]
        pub const fn wdc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog configuration"]
        #[inline(always)]
        pub fn set_wdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Watchdog value"]
        #[inline(always)]
        pub const fn wdv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog value"]
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
    #[doc = "FDCAN Rx Buffer Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxbc(pub u32);
    impl Rxbc {
        #[doc = "Rx Buffer Start Address"]
        #[inline(always)]
        pub const fn rbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx Buffer Start Address"]
        #[inline(always)]
        pub fn set_rbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
    }
    impl Default for Rxbc {
        #[inline(always)]
        fn default() -> Rxbc {
            Rxbc(0)
        }
    }
    impl core::fmt::Debug for Rxbc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxbc").field("rbsa", &self.rbsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxbc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxbc {{ rbsa: {=u16:?} }}", self.rbsa())
        }
    }
    #[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxesc(pub u32);
    impl Rxesc {
        #[doc = "Rx FIFO X Data Field Size"]
        #[inline(always)]
        pub const fn fds(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Rx FIFO X Data Field Size"]
        #[inline(always)]
        pub fn set_fds(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
        #[doc = "Rx Buffer Data Field Size"]
        #[inline(always)]
        pub const fn rbds(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Rx Buffer Data Field Size"]
        #[inline(always)]
        pub fn set_rbds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Rxesc {
        #[inline(always)]
        fn default() -> Rxesc {
            Rxesc(0)
        }
    }
    impl core::fmt::Debug for Rxesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxesc")
                .field("fds[0]", &self.fds(0usize))
                .field("fds[1]", &self.fds(1usize))
                .field("rbds", &self.rbds())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxesc {{ fds[0]: {=u8:?}, fds[1]: {=u8:?}, rbds: {=u8:?} }}",
                self.fds(0usize),
                self.fds(1usize),
                self.rbds()
            )
        }
    }
    #[doc = "CAN Rx FIFO X Acknowledge Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfa(pub u32);
    impl Rxfa {
        #[doc = "Rx FIFO X Acknowledge Index"]
        #[inline(always)]
        pub const fn fai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO X Acknowledge Index"]
        #[inline(always)]
        pub fn set_fai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
    #[doc = "FDCAN Rx FIFO X Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfc(pub u32);
    impl Rxfc {
        #[doc = "Rx FIFO X Start Address"]
        #[inline(always)]
        pub const fn fsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx FIFO X Start Address"]
        #[inline(always)]
        pub fn set_fsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Rx FIFO X Size"]
        #[inline(always)]
        pub const fn fs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO X Size"]
        #[inline(always)]
        pub fn set_fs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "FIFO X Watermark"]
        #[inline(always)]
        pub const fn fwm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "FIFO X Watermark"]
        #[inline(always)]
        pub fn set_fwm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "FIFO X operation mode"]
        #[inline(always)]
        pub const fn fom(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO X operation mode"]
        #[inline(always)]
        pub fn set_fom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rxfc {
        #[inline(always)]
        fn default() -> Rxfc {
            Rxfc(0)
        }
    }
    impl core::fmt::Debug for Rxfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxfc")
                .field("fsa", &self.fsa())
                .field("fs", &self.fs())
                .field("fwm", &self.fwm())
                .field("fom", &self.fom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxfc {{ fsa: {=u16:?}, fs: {=u8:?}, fwm: {=u8:?}, fom: {=bool:?} }}",
                self.fsa(),
                self.fs(),
                self.fwm(),
                self.fom()
            )
        }
    }
    #[doc = "FDCAN Rx FIFO X Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfs(pub u32);
    impl Rxfs {
        #[doc = "Rx FIFO X Fill Level"]
        #[inline(always)]
        pub const fn ffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO X Fill Level"]
        #[inline(always)]
        pub fn set_ffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Rx FIFO X Get Index"]
        #[inline(always)]
        pub const fn fgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO X Get Index"]
        #[inline(always)]
        pub fn set_fgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Rx FIFO X Put Index"]
        #[inline(always)]
        pub const fn fpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO X Put Index"]
        #[inline(always)]
        pub fn set_fpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub const fn ff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub fn set_ff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub const fn rfl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost"]
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
    #[doc = "FDCAN Standard ID Filter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidfc(pub u32);
    impl Sidfc {
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub const fn flssa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub fn set_flssa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "List Size Standard"]
        #[inline(always)]
        pub const fn lss(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "List Size Standard"]
        #[inline(always)]
        pub fn set_lss(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Sidfc {
        #[inline(always)]
        fn default() -> Sidfc {
            Sidfc(0)
        }
    }
    impl core::fmt::Debug for Sidfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sidfc")
                .field("flssa", &self.flssa())
                .field("lss", &self.lss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sidfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sidfc {{ flssa: {=u16:?}, lss: {=u8:?} }}", self.flssa(), self.lss())
        }
    }
    #[doc = "FDCAN Transmitter Delay Compensation Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdcr(pub u32);
    impl Tdcr {
        #[doc = "Transmitter Delay Compensation Filter Window Length"]
        #[inline(always)]
        pub const fn tdcf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Filter Window Length"]
        #[inline(always)]
        pub fn set_tdcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Transmitter Delay Compensation Offset"]
        #[inline(always)]
        pub const fn tdco(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Offset"]
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
    #[doc = "FDCAN Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Test(pub u32);
    impl Test {
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub const fn lbck(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub fn set_lbck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Control of Transmit Pin"]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Control of Transmit Pin"]
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
                "Test {{ lbck: {=bool:?}, tx: {=u8:?}, rx: {=bool:?} }}",
                self.lbck(),
                self.tx(),
                self.rx()
            )
        }
    }
    #[doc = "FDCAN Timeout Counter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocc(pub u32);
    impl Tocc {
        #[doc = "Enable Timeout Counter"]
        #[inline(always)]
        pub const fn etoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timeout Counter"]
        #[inline(always)]
        pub fn set_etoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout Select"]
        #[inline(always)]
        pub const fn tos(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Timeout Select"]
        #[inline(always)]
        pub fn set_tos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Timeout Period"]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout Period"]
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
                "Tocc {{ etoc: {=bool:?}, tos: {=u8:?}, top: {=u16:?} }}",
                self.etoc(),
                self.tos(),
                self.top()
            )
        }
    }
    #[doc = "FDCAN Timeout Counter Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocv(pub u32);
    impl Tocv {
        #[doc = "Timeout Counter"]
        #[inline(always)]
        pub const fn toc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout Counter"]
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
    #[doc = "FDCAN Timestamp Counter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscc(pub u32);
    impl Tscc {
        #[doc = "Timestamp Select"]
        #[inline(always)]
        pub const fn tss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Timestamp Select"]
        #[inline(always)]
        pub fn set_tss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Timestamp Counter Prescaler"]
        #[inline(always)]
        pub const fn tcp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Counter Prescaler"]
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
            defmt::write!(f, "Tscc {{ tss: {=u8:?}, tcp: {=u8:?} }}", self.tss(), self.tcp())
        }
    }
    #[doc = "FDCAN Timestamp Counter Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscv(pub u32);
    impl Tscv {
        #[doc = "Timestamp Counter"]
        #[inline(always)]
        pub const fn tsc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp Counter"]
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
    #[doc = "FDCAN TT Capture Time Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcpt(pub u32);
    impl Ttcpt {
        #[doc = "Cycle Count Value"]
        #[inline(always)]
        pub const fn ccv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Cycle Count Value"]
        #[inline(always)]
        pub fn set_ccv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Stop Watch Value"]
        #[inline(always)]
        pub const fn swv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Stop Watch Value"]
        #[inline(always)]
        pub fn set_swv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ttcpt {
        #[inline(always)]
        fn default() -> Ttcpt {
            Ttcpt(0)
        }
    }
    impl core::fmt::Debug for Ttcpt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttcpt")
                .field("ccv", &self.ccv())
                .field("swv", &self.swv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttcpt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ttcpt {{ ccv: {=u8:?}, swv: {=u16:?} }}", self.ccv(), self.swv())
        }
    }
    #[doc = "FDCAN TT Cycle Sync Mark Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcsm(pub u32);
    impl Ttcsm {
        #[doc = "Cycle Sync Mark"]
        #[inline(always)]
        pub const fn csm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Cycle Sync Mark"]
        #[inline(always)]
        pub fn set_csm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ttcsm {
        #[inline(always)]
        fn default() -> Ttcsm {
            Ttcsm(0)
        }
    }
    impl core::fmt::Debug for Ttcsm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttcsm").field("csm", &self.csm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttcsm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ttcsm {{ csm: {=u16:?} }}", self.csm())
        }
    }
    #[doc = "FDCAN TT Cycle Time and Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttctc(pub u32);
    impl Ttctc {
        #[doc = "Cycle Time"]
        #[inline(always)]
        pub const fn ct(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Cycle Time"]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Cycle Count"]
        #[inline(always)]
        pub const fn cc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Cycle Count"]
        #[inline(always)]
        pub fn set_cc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ttctc {
        #[inline(always)]
        fn default() -> Ttctc {
            Ttctc(0)
        }
    }
    impl core::fmt::Debug for Ttctc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttctc")
                .field("ct", &self.ct())
                .field("cc", &self.cc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttctc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ttctc {{ ct: {=u16:?}, cc: {=u8:?} }}", self.ct(), self.cc())
        }
    }
    #[doc = "FDCAN TT Global Time Preset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttgtp(pub u32);
    impl Ttgtp {
        #[doc = "Time Preset"]
        #[inline(always)]
        pub const fn ncl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Time Preset"]
        #[inline(always)]
        pub fn set_ncl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Cycle Time Target Phase"]
        #[inline(always)]
        pub const fn ctp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Cycle Time Target Phase"]
        #[inline(always)]
        pub fn set_ctp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ttgtp {
        #[inline(always)]
        fn default() -> Ttgtp {
            Ttgtp(0)
        }
    }
    impl core::fmt::Debug for Ttgtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttgtp")
                .field("ncl", &self.ncl())
                .field("ctp", &self.ctp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttgtp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ttgtp {{ ncl: {=u16:?}, ctp: {=u16:?} }}", self.ncl(), self.ctp())
        }
    }
    #[doc = "FDCAN TT Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttie(pub u32);
    impl Ttie {
        #[doc = "Start of Basic Cycle Interrupt Enable"]
        #[inline(always)]
        pub const fn sbce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Basic Cycle Interrupt Enable"]
        #[inline(always)]
        pub fn set_sbce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of Matrix Cycle Interrupt Enable"]
        #[inline(always)]
        pub const fn smce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Matrix Cycle Interrupt Enable"]
        #[inline(always)]
        pub fn set_smce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Change of Synchronization Mode Interrupt Enable"]
        #[inline(always)]
        pub const fn csme(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Change of Synchronization Mode Interrupt Enable"]
        #[inline(always)]
        pub fn set_csme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Gap Interrupt Enable"]
        #[inline(always)]
        pub const fn soge(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Gap Interrupt Enable"]
        #[inline(always)]
        pub fn set_soge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt Enable"]
        #[inline(always)]
        pub const fn rtmie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt Enable"]
        #[inline(always)]
        pub fn set_rtmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Enable"]
        #[inline(always)]
        pub const fn ttmie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Enable"]
        #[inline(always)]
        pub fn set_ttmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Stop Watch Event Interrupt Enable"]
        #[inline(always)]
        pub const fn swee(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Event Interrupt Enable"]
        #[inline(always)]
        pub fn set_swee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Time Wrap Interrupt Enable"]
        #[inline(always)]
        pub const fn gtwe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Wrap Interrupt Enable"]
        #[inline(always)]
        pub fn set_gtwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global Time Discontinuity Interrupt Enable"]
        #[inline(always)]
        pub const fn gtde(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Discontinuity Interrupt Enable"]
        #[inline(always)]
        pub fn set_gtde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Global Time Error Interrupt Enable"]
        #[inline(always)]
        pub const fn gtee(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_gtee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx Count Underflow Interrupt Enable"]
        #[inline(always)]
        pub const fn txue(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Underflow Interrupt Enable"]
        #[inline(always)]
        pub fn set_txue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx Count Overflow Interrupt Enable"]
        #[inline(always)]
        pub const fn txoe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn set_txoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Scheduling Error 1 Interrupt Enable"]
        #[inline(always)]
        pub const fn se1e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 1 Interrupt Enable"]
        #[inline(always)]
        pub fn set_se1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Scheduling Error 2 Interrupt Enable"]
        #[inline(always)]
        pub const fn se2e(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 2 Interrupt Enable"]
        #[inline(always)]
        pub fn set_se2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Change Error Level Interrupt Enable"]
        #[inline(always)]
        pub const fn elce(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Change Error Level Interrupt Enable"]
        #[inline(always)]
        pub fn set_elce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Initialization Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub const fn iwtge(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub fn set_iwtge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub const fn wte(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub fn set_wte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Application Watchdog Interrupt Enable"]
        #[inline(always)]
        pub const fn awe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog Interrupt Enable"]
        #[inline(always)]
        pub fn set_awe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Configuration Error Interrupt Enable"]
        #[inline(always)]
        pub const fn cere(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_cere(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ttie {
        #[inline(always)]
        fn default() -> Ttie {
            Ttie(0)
        }
    }
    impl core::fmt::Debug for Ttie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttie")
                .field("sbce", &self.sbce())
                .field("smce", &self.smce())
                .field("csme", &self.csme())
                .field("soge", &self.soge())
                .field("rtmie", &self.rtmie())
                .field("ttmie", &self.ttmie())
                .field("swee", &self.swee())
                .field("gtwe", &self.gtwe())
                .field("gtde", &self.gtde())
                .field("gtee", &self.gtee())
                .field("txue", &self.txue())
                .field("txoe", &self.txoe())
                .field("se1e", &self.se1e())
                .field("se2e", &self.se2e())
                .field("elce", &self.elce())
                .field("iwtge", &self.iwtge())
                .field("wte", &self.wte())
                .field("awe", &self.awe())
                .field("cere", &self.cere())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttie {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ttie {{ sbce: {=bool:?}, smce: {=bool:?}, csme: {=bool:?}, soge: {=bool:?}, rtmie: {=bool:?}, ttmie: {=bool:?}, swee: {=bool:?}, gtwe: {=bool:?}, gtde: {=bool:?}, gtee: {=bool:?}, txue: {=bool:?}, txoe: {=bool:?}, se1e: {=bool:?}, se2e: {=bool:?}, elce: {=bool:?}, iwtge: {=bool:?}, wte: {=bool:?}, awe: {=bool:?}, cere: {=bool:?} }}" , self . sbce () , self . smce () , self . csme () , self . soge () , self . rtmie () , self . ttmie () , self . swee () , self . gtwe () , self . gtde () , self . gtee () , self . txue () , self . txoe () , self . se1e () , self . se2e () , self . elce () , self . iwtge () , self . wte () , self . awe () , self . cere ())
        }
    }
    #[doc = "FDCAN TT Interrupt Line Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttils(pub u32);
    impl Ttils {
        #[doc = "Start of Basic Cycle Interrupt Line"]
        #[inline(always)]
        pub const fn sbcl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Basic Cycle Interrupt Line"]
        #[inline(always)]
        pub fn set_sbcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of Matrix Cycle Interrupt Line"]
        #[inline(always)]
        pub const fn smcl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Matrix Cycle Interrupt Line"]
        #[inline(always)]
        pub fn set_smcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Change of Synchronization Mode Interrupt Line"]
        #[inline(always)]
        pub const fn csml(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Change of Synchronization Mode Interrupt Line"]
        #[inline(always)]
        pub fn set_csml(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Gap Interrupt Line"]
        #[inline(always)]
        pub const fn sogl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Gap Interrupt Line"]
        #[inline(always)]
        pub fn set_sogl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt Line"]
        #[inline(always)]
        pub const fn rtmil(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt Line"]
        #[inline(always)]
        pub fn set_rtmil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Line"]
        #[inline(always)]
        pub const fn ttmil(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Line"]
        #[inline(always)]
        pub fn set_ttmil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Stop Watch Event Interrupt Line"]
        #[inline(always)]
        pub const fn swel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Event Interrupt Line"]
        #[inline(always)]
        pub fn set_swel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Time Wrap Interrupt Line"]
        #[inline(always)]
        pub const fn gtwl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Wrap Interrupt Line"]
        #[inline(always)]
        pub fn set_gtwl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global Time Discontinuity Interrupt Line"]
        #[inline(always)]
        pub const fn gtdl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Discontinuity Interrupt Line"]
        #[inline(always)]
        pub fn set_gtdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Global Time Error Interrupt Line"]
        #[inline(always)]
        pub const fn gtel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Error Interrupt Line"]
        #[inline(always)]
        pub fn set_gtel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx Count Underflow Interrupt Line"]
        #[inline(always)]
        pub const fn txul(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Underflow Interrupt Line"]
        #[inline(always)]
        pub fn set_txul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx Count Overflow Interrupt Line"]
        #[inline(always)]
        pub const fn txol(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Overflow Interrupt Line"]
        #[inline(always)]
        pub fn set_txol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Scheduling Error 1 Interrupt Line"]
        #[inline(always)]
        pub const fn se1l(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 1 Interrupt Line"]
        #[inline(always)]
        pub fn set_se1l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Scheduling Error 2 Interrupt Line"]
        #[inline(always)]
        pub const fn se2l(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 2 Interrupt Line"]
        #[inline(always)]
        pub fn set_se2l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Change Error Level Interrupt Line"]
        #[inline(always)]
        pub const fn elcl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Change Error Level Interrupt Line"]
        #[inline(always)]
        pub fn set_elcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Initialization Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub const fn iwtgl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub fn set_iwtgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub const fn wtl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub fn set_wtl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Application Watchdog Interrupt Line"]
        #[inline(always)]
        pub const fn awl(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog Interrupt Line"]
        #[inline(always)]
        pub fn set_awl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Configuration Error Interrupt Line"]
        #[inline(always)]
        pub const fn cerl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error Interrupt Line"]
        #[inline(always)]
        pub fn set_cerl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ttils {
        #[inline(always)]
        fn default() -> Ttils {
            Ttils(0)
        }
    }
    impl core::fmt::Debug for Ttils {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttils")
                .field("sbcl", &self.sbcl())
                .field("smcl", &self.smcl())
                .field("csml", &self.csml())
                .field("sogl", &self.sogl())
                .field("rtmil", &self.rtmil())
                .field("ttmil", &self.ttmil())
                .field("swel", &self.swel())
                .field("gtwl", &self.gtwl())
                .field("gtdl", &self.gtdl())
                .field("gtel", &self.gtel())
                .field("txul", &self.txul())
                .field("txol", &self.txol())
                .field("se1l", &self.se1l())
                .field("se2l", &self.se2l())
                .field("elcl", &self.elcl())
                .field("iwtgl", &self.iwtgl())
                .field("wtl", &self.wtl())
                .field("awl", &self.awl())
                .field("cerl", &self.cerl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttils {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ttils {{ sbcl: {=bool:?}, smcl: {=bool:?}, csml: {=bool:?}, sogl: {=bool:?}, rtmil: {=bool:?}, ttmil: {=bool:?}, swel: {=bool:?}, gtwl: {=bool:?}, gtdl: {=bool:?}, gtel: {=bool:?}, txul: {=bool:?}, txol: {=bool:?}, se1l: {=bool:?}, se2l: {=bool:?}, elcl: {=bool:?}, iwtgl: {=bool:?}, wtl: {=bool:?}, awl: {=bool:?}, cerl: {=bool:?} }}" , self . sbcl () , self . smcl () , self . csml () , self . sogl () , self . rtmil () , self . ttmil () , self . swel () , self . gtwl () , self . gtdl () , self . gtel () , self . txul () , self . txol () , self . se1l () , self . se2l () , self . elcl () , self . iwtgl () , self . wtl () , self . awl () , self . cerl ())
        }
    }
    #[doc = "FDCAN TT Interrupt Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttir(pub u32);
    impl Ttir {
        #[doc = "Start of Basic Cycle"]
        #[inline(always)]
        pub const fn sbc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Basic Cycle"]
        #[inline(always)]
        pub fn set_sbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of Matrix Cycle"]
        #[inline(always)]
        pub const fn smc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Matrix Cycle"]
        #[inline(always)]
        pub fn set_smc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Change of Synchronization Mode"]
        #[inline(always)]
        pub const fn csm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Change of Synchronization Mode"]
        #[inline(always)]
        pub fn set_csm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Gap"]
        #[inline(always)]
        pub const fn sog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Gap"]
        #[inline(always)]
        pub fn set_sog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt"]
        #[inline(always)]
        pub const fn rtmi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt"]
        #[inline(always)]
        pub fn set_rtmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trigger Time Mark Event Internal"]
        #[inline(always)]
        pub const fn ttmi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Event Internal"]
        #[inline(always)]
        pub fn set_ttmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Stop Watch Event"]
        #[inline(always)]
        pub const fn swe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Event"]
        #[inline(always)]
        pub fn set_swe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Time Wrap"]
        #[inline(always)]
        pub const fn gtw(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Wrap"]
        #[inline(always)]
        pub fn set_gtw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global Time Discontinuity"]
        #[inline(always)]
        pub const fn gtd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Discontinuity"]
        #[inline(always)]
        pub fn set_gtd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Global Time Error"]
        #[inline(always)]
        pub const fn gte(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Error"]
        #[inline(always)]
        pub fn set_gte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx Count Underflow"]
        #[inline(always)]
        pub const fn txu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Underflow"]
        #[inline(always)]
        pub fn set_txu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx Count Overflow"]
        #[inline(always)]
        pub const fn txo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Overflow"]
        #[inline(always)]
        pub fn set_txo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Scheduling Error 1"]
        #[inline(always)]
        pub const fn se1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 1"]
        #[inline(always)]
        pub fn set_se1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Scheduling Error 2"]
        #[inline(always)]
        pub const fn se2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 2"]
        #[inline(always)]
        pub fn set_se2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Error Level Changed"]
        #[inline(always)]
        pub const fn elc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Error Level Changed"]
        #[inline(always)]
        pub fn set_elc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Initialization Watch Trigger"]
        #[inline(always)]
        pub const fn iwtg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Watch Trigger"]
        #[inline(always)]
        pub fn set_iwtg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Watch Trigger"]
        #[inline(always)]
        pub const fn wt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger"]
        #[inline(always)]
        pub fn set_wt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Application Watchdog"]
        #[inline(always)]
        pub const fn aw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog"]
        #[inline(always)]
        pub fn set_aw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Configuration Error"]
        #[inline(always)]
        pub const fn cer(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error"]
        #[inline(always)]
        pub fn set_cer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ttir {
        #[inline(always)]
        fn default() -> Ttir {
            Ttir(0)
        }
    }
    impl core::fmt::Debug for Ttir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttir")
                .field("sbc", &self.sbc())
                .field("smc", &self.smc())
                .field("csm", &self.csm())
                .field("sog", &self.sog())
                .field("rtmi", &self.rtmi())
                .field("ttmi", &self.ttmi())
                .field("swe", &self.swe())
                .field("gtw", &self.gtw())
                .field("gtd", &self.gtd())
                .field("gte", &self.gte())
                .field("txu", &self.txu())
                .field("txo", &self.txo())
                .field("se1", &self.se1())
                .field("se2", &self.se2())
                .field("elc", &self.elc())
                .field("iwtg", &self.iwtg())
                .field("wt", &self.wt())
                .field("aw", &self.aw())
                .field("cer", &self.cer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttir {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ttir {{ sbc: {=bool:?}, smc: {=bool:?}, csm: {=bool:?}, sog: {=bool:?}, rtmi: {=bool:?}, ttmi: {=bool:?}, swe: {=bool:?}, gtw: {=bool:?}, gtd: {=bool:?}, gte: {=bool:?}, txu: {=bool:?}, txo: {=bool:?}, se1: {=bool:?}, se2: {=bool:?}, elc: {=bool:?}, iwtg: {=bool:?}, wt: {=bool:?}, aw: {=bool:?}, cer: {=bool:?} }}" , self . sbc () , self . smc () , self . csm () , self . sog () , self . rtmi () , self . ttmi () , self . swe () , self . gtw () , self . gtd () , self . gte () , self . txu () , self . txo () , self . se1 () , self . se2 () , self . elc () , self . iwtg () , self . wt () , self . aw () , self . cer ())
        }
    }
    #[doc = "FDCAN TT Local and Global Time Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttlgt(pub u32);
    impl Ttlgt {
        #[doc = "Local Time"]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Local Time"]
        #[inline(always)]
        pub fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Global Time"]
        #[inline(always)]
        pub const fn gt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Global Time"]
        #[inline(always)]
        pub fn set_gt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ttlgt {
        #[inline(always)]
        fn default() -> Ttlgt {
            Ttlgt(0)
        }
    }
    impl core::fmt::Debug for Ttlgt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttlgt")
                .field("lt", &self.lt())
                .field("gt", &self.gt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttlgt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ttlgt {{ lt: {=u16:?}, gt: {=u16:?} }}", self.lt(), self.gt())
        }
    }
    #[doc = "FDCAN TT Matrix Limits Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttmlm(pub u32);
    impl Ttmlm {
        #[doc = "Cycle Count Max"]
        #[inline(always)]
        pub const fn ccm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Cycle Count Max"]
        #[inline(always)]
        pub fn set_ccm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Cycle Start Synchronization"]
        #[inline(always)]
        pub const fn css(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Cycle Start Synchronization"]
        #[inline(always)]
        pub fn set_css(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Tx Enable Window"]
        #[inline(always)]
        pub const fn txew(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Tx Enable Window"]
        #[inline(always)]
        pub fn set_txew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Expected Number of Tx Triggers"]
        #[inline(always)]
        pub const fn entt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Expected Number of Tx Triggers"]
        #[inline(always)]
        pub fn set_entt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Ttmlm {
        #[inline(always)]
        fn default() -> Ttmlm {
            Ttmlm(0)
        }
    }
    impl core::fmt::Debug for Ttmlm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttmlm")
                .field("ccm", &self.ccm())
                .field("css", &self.css())
                .field("txew", &self.txew())
                .field("entt", &self.entt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttmlm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ttmlm {{ ccm: {=u8:?}, css: {=u8:?}, txew: {=u8:?}, entt: {=u16:?} }}",
                self.ccm(),
                self.css(),
                self.txew(),
                self.entt()
            )
        }
    }
    #[doc = "FDCAN TT Operation Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttocf(pub u32);
    impl Ttocf {
        #[doc = "Operation Mode"]
        #[inline(always)]
        pub const fn om(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Operation Mode"]
        #[inline(always)]
        pub fn set_om(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Gap Enable"]
        #[inline(always)]
        pub const fn gaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Enable"]
        #[inline(always)]
        pub fn set_gaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Time Master"]
        #[inline(always)]
        pub const fn tm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Time Master"]
        #[inline(always)]
        pub fn set_tm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LD of Synchronization Deviation Limit"]
        #[inline(always)]
        pub const fn ldsdl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "LD of Synchronization Deviation Limit"]
        #[inline(always)]
        pub fn set_ldsdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "Initial Reference Trigger Offset"]
        #[inline(always)]
        pub const fn irto(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Initial Reference Trigger Offset"]
        #[inline(always)]
        pub fn set_irto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Enable External Clock Synchronization"]
        #[inline(always)]
        pub const fn eecs(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable External Clock Synchronization"]
        #[inline(always)]
        pub fn set_eecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Application Watchdog Limit"]
        #[inline(always)]
        pub const fn awl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Application Watchdog Limit"]
        #[inline(always)]
        pub fn set_awl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Enable Global Time Filtering"]
        #[inline(always)]
        pub const fn egtf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Global Time Filtering"]
        #[inline(always)]
        pub fn set_egtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Enable Clock Calibration"]
        #[inline(always)]
        pub const fn ecc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Clock Calibration"]
        #[inline(always)]
        pub fn set_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Event Trigger Polarity"]
        #[inline(always)]
        pub const fn evtp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Event Trigger Polarity"]
        #[inline(always)]
        pub fn set_evtp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Ttocf {
        #[inline(always)]
        fn default() -> Ttocf {
            Ttocf(0)
        }
    }
    impl core::fmt::Debug for Ttocf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttocf")
                .field("om", &self.om())
                .field("gaen", &self.gaen())
                .field("tm", &self.tm())
                .field("ldsdl", &self.ldsdl())
                .field("irto", &self.irto())
                .field("eecs", &self.eecs())
                .field("awl", &self.awl())
                .field("egtf", &self.egtf())
                .field("ecc", &self.ecc())
                .field("evtp", &self.evtp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttocf {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ttocf {{ om: {=u8:?}, gaen: {=bool:?}, tm: {=bool:?}, ldsdl: {=u8:?}, irto: {=u8:?}, eecs: {=bool:?}, awl: {=u8:?}, egtf: {=bool:?}, ecc: {=bool:?}, evtp: {=bool:?} }}" , self . om () , self . gaen () , self . tm () , self . ldsdl () , self . irto () , self . eecs () , self . awl () , self . egtf () , self . ecc () , self . evtp ())
        }
    }
    #[doc = "FDCAN TT Operation Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttocn(pub u32);
    impl Ttocn {
        #[doc = "Set Global time"]
        #[inline(always)]
        pub const fn sgt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Set Global time"]
        #[inline(always)]
        pub fn set_sgt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Clock Synchronization"]
        #[inline(always)]
        pub const fn ecs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Clock Synchronization"]
        #[inline(always)]
        pub fn set_ecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Stop Watch Polarity"]
        #[inline(always)]
        pub const fn swp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Polarity"]
        #[inline(always)]
        pub fn set_swp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Stop Watch Source"]
        #[inline(always)]
        pub const fn sws(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Stop Watch Source"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub const fn rtie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub fn set_rtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Register Time Mark Compare"]
        #[inline(always)]
        pub const fn tmc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Register Time Mark Compare"]
        #[inline(always)]
        pub fn set_tmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Trigger Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub const fn ttie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub fn set_ttie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Gap Control Select"]
        #[inline(always)]
        pub const fn gcs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Control Select"]
        #[inline(always)]
        pub fn set_gcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Finish Gap"]
        #[inline(always)]
        pub const fn fgp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Finish Gap"]
        #[inline(always)]
        pub fn set_fgp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Time Mark Gap"]
        #[inline(always)]
        pub const fn tmg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Time Mark Gap"]
        #[inline(always)]
        pub fn set_tmg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Next is Gap"]
        #[inline(always)]
        pub const fn nig(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Next is Gap"]
        #[inline(always)]
        pub fn set_nig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "External Synchronization Control"]
        #[inline(always)]
        pub const fn escn(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "External Synchronization Control"]
        #[inline(always)]
        pub fn set_escn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TT Operation Control Register Locked"]
        #[inline(always)]
        pub const fn lckc(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TT Operation Control Register Locked"]
        #[inline(always)]
        pub fn set_lckc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ttocn {
        #[inline(always)]
        fn default() -> Ttocn {
            Ttocn(0)
        }
    }
    impl core::fmt::Debug for Ttocn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttocn")
                .field("sgt", &self.sgt())
                .field("ecs", &self.ecs())
                .field("swp", &self.swp())
                .field("sws", &self.sws())
                .field("rtie", &self.rtie())
                .field("tmc", &self.tmc())
                .field("ttie", &self.ttie())
                .field("gcs", &self.gcs())
                .field("fgp", &self.fgp())
                .field("tmg", &self.tmg())
                .field("nig", &self.nig())
                .field("escn", &self.escn())
                .field("lckc", &self.lckc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttocn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ttocn {{ sgt: {=bool:?}, ecs: {=bool:?}, swp: {=bool:?}, sws: {=u8:?}, rtie: {=bool:?}, tmc: {=u8:?}, ttie: {=bool:?}, gcs: {=bool:?}, fgp: {=bool:?}, tmg: {=bool:?}, nig: {=bool:?}, escn: {=bool:?}, lckc: {=bool:?} }}" , self . sgt () , self . ecs () , self . swp () , self . sws () , self . rtie () , self . tmc () , self . ttie () , self . gcs () , self . fgp () , self . tmg () , self . nig () , self . escn () , self . lckc ())
        }
    }
    #[doc = "FDCAN TT Operation Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttost(pub u32);
    impl Ttost {
        #[doc = "Error Level"]
        #[inline(always)]
        pub const fn el(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Error Level"]
        #[inline(always)]
        pub fn set_el(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Master State"]
        #[inline(always)]
        pub const fn ms(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Master State"]
        #[inline(always)]
        pub fn set_ms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Synchronization State"]
        #[inline(always)]
        pub const fn sys(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization State"]
        #[inline(always)]
        pub fn set_sys(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Quality of Global Time Phase"]
        #[inline(always)]
        pub const fn qgtp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Quality of Global Time Phase"]
        #[inline(always)]
        pub fn set_qgtp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Quality of Clock Speed"]
        #[inline(always)]
        pub const fn qcs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Quality of Clock Speed"]
        #[inline(always)]
        pub fn set_qcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Reference Trigger Offset"]
        #[inline(always)]
        pub const fn rto(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Reference Trigger Offset"]
        #[inline(always)]
        pub fn set_rto(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Wait for Global Time Discontinuity"]
        #[inline(always)]
        pub const fn wgtd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for Global Time Discontinuity"]
        #[inline(always)]
        pub fn set_wgtd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Gap Finished Indicator"]
        #[inline(always)]
        pub const fn gfi(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Finished Indicator"]
        #[inline(always)]
        pub fn set_gfi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Time Master Priority"]
        #[inline(always)]
        pub const fn tmp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Time Master Priority"]
        #[inline(always)]
        pub fn set_tmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Gap Started Indicator"]
        #[inline(always)]
        pub const fn gsi(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Started Indicator"]
        #[inline(always)]
        pub fn set_gsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Wait for Event"]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for Event"]
        #[inline(always)]
        pub fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Application Watchdog Event"]
        #[inline(always)]
        pub const fn awe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog Event"]
        #[inline(always)]
        pub fn set_awe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Wait for External Clock Synchronization"]
        #[inline(always)]
        pub const fn wecs(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for External Clock Synchronization"]
        #[inline(always)]
        pub fn set_wecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Schedule Phase Lock"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Schedule Phase Lock"]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ttost {
        #[inline(always)]
        fn default() -> Ttost {
            Ttost(0)
        }
    }
    impl core::fmt::Debug for Ttost {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttost")
                .field("el", &self.el())
                .field("ms", &self.ms())
                .field("sys", &self.sys())
                .field("qgtp", &self.qgtp())
                .field("qcs", &self.qcs())
                .field("rto", &self.rto())
                .field("wgtd", &self.wgtd())
                .field("gfi", &self.gfi())
                .field("tmp", &self.tmp())
                .field("gsi", &self.gsi())
                .field("wfe", &self.wfe())
                .field("awe", &self.awe())
                .field("wecs", &self.wecs())
                .field("spl", &self.spl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttost {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ttost {{ el: {=u8:?}, ms: {=u8:?}, sys: {=u8:?}, qgtp: {=bool:?}, qcs: {=bool:?}, rto: {=u8:?}, wgtd: {=bool:?}, gfi: {=bool:?}, tmp: {=u8:?}, gsi: {=bool:?}, wfe: {=bool:?}, awe: {=bool:?}, wecs: {=bool:?}, spl: {=bool:?} }}" , self . el () , self . ms () , self . sys () , self . qgtp () , self . qcs () , self . rto () , self . wgtd () , self . gfi () , self . tmp () , self . gsi () , self . wfe () , self . awe () , self . wecs () , self . spl ())
        }
    }
    #[doc = "FDCAN TT Reference Message Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttrmc(pub u32);
    impl Ttrmc {
        #[doc = "Reference Identifier"]
        #[inline(always)]
        pub const fn rid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Reference Identifier"]
        #[inline(always)]
        pub fn set_rid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[doc = "Extended Identifier"]
        #[inline(always)]
        pub const fn xtd(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Extended Identifier"]
        #[inline(always)]
        pub fn set_xtd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Reference Message Payload Select"]
        #[inline(always)]
        pub const fn rmps(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reference Message Payload Select"]
        #[inline(always)]
        pub fn set_rmps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ttrmc {
        #[inline(always)]
        fn default() -> Ttrmc {
            Ttrmc(0)
        }
    }
    impl core::fmt::Debug for Ttrmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttrmc")
                .field("rid", &self.rid())
                .field("xtd", &self.xtd())
                .field("rmps", &self.rmps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttrmc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ttrmc {{ rid: {=u32:?}, xtd: {=bool:?}, rmps: {=bool:?} }}",
                self.rid(),
                self.xtd(),
                self.rmps()
            )
        }
    }
    #[doc = "FDCAN TT Trigger Memory Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tttmc(pub u32);
    impl Tttmc {
        #[doc = "Trigger Memory Start Address"]
        #[inline(always)]
        pub const fn tmsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Trigger Memory Start Address"]
        #[inline(always)]
        pub fn set_tmsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Trigger Memory Elements"]
        #[inline(always)]
        pub const fn tme(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Trigger Memory Elements"]
        #[inline(always)]
        pub fn set_tme(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Tttmc {
        #[inline(always)]
        fn default() -> Tttmc {
            Tttmc(0)
        }
    }
    impl core::fmt::Debug for Tttmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tttmc")
                .field("tmsa", &self.tmsa())
                .field("tme", &self.tme())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tttmc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tttmc {{ tmsa: {=u16:?}, tme: {=u8:?} }}", self.tmsa(), self.tme())
        }
    }
    #[doc = "FDCAN TT Time Mark Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tttmk(pub u32);
    impl Tttmk {
        #[doc = "Time Mark"]
        #[inline(always)]
        pub const fn tm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Time Mark"]
        #[inline(always)]
        pub fn set_tm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Time Mark Cycle Code"]
        #[inline(always)]
        pub const fn ticc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Time Mark Cycle Code"]
        #[inline(always)]
        pub fn set_ticc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "TT Time Mark Register Locked"]
        #[inline(always)]
        pub const fn lckm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TT Time Mark Register Locked"]
        #[inline(always)]
        pub fn set_lckm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Tttmk {
        #[inline(always)]
        fn default() -> Tttmk {
            Tttmk(0)
        }
    }
    impl core::fmt::Debug for Tttmk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tttmk")
                .field("tm", &self.tm())
                .field("ticc", &self.ticc())
                .field("lckm", &self.lckm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tttmk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tttmk {{ tm: {=u16:?}, ticc: {=u8:?}, lckm: {=bool:?} }}",
                self.tm(),
                self.ticc(),
                self.lckm()
            )
        }
    }
    #[doc = "FDCAN TT Trigger Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttts(pub u32);
    impl Ttts {
        #[doc = "Stop watch trigger input selection"]
        #[inline(always)]
        pub const fn swtdel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Stop watch trigger input selection"]
        #[inline(always)]
        pub fn set_swtdel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Event trigger input selection"]
        #[inline(always)]
        pub const fn evtsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Event trigger input selection"]
        #[inline(always)]
        pub fn set_evtsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Ttts {
        #[inline(always)]
        fn default() -> Ttts {
            Ttts(0)
        }
    }
    impl core::fmt::Debug for Ttts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ttts")
                .field("swtdel", &self.swtdel())
                .field("evtsel", &self.evtsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ttts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ttts {{ swtdel: {=u8:?}, evtsel: {=u8:?} }}",
                self.swtdel(),
                self.evtsel()
            )
        }
    }
    #[doc = "FDCAN TUR Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Turcf(pub u32);
    impl Turcf {
        #[doc = "Numerator Configuration Low"]
        #[inline(always)]
        pub const fn ncl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Numerator Configuration Low"]
        #[inline(always)]
        pub fn set_ncl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Denominator Configuration"]
        #[inline(always)]
        pub const fn dc(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Denominator Configuration"]
        #[inline(always)]
        pub fn set_dc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
        #[doc = "Enable Local Time"]
        #[inline(always)]
        pub const fn elt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Local Time"]
        #[inline(always)]
        pub fn set_elt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Turcf {
        #[inline(always)]
        fn default() -> Turcf {
            Turcf(0)
        }
    }
    impl core::fmt::Debug for Turcf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Turcf")
                .field("ncl", &self.ncl())
                .field("dc", &self.dc())
                .field("elt", &self.elt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Turcf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Turcf {{ ncl: {=u16:?}, dc: {=u16:?}, elt: {=bool:?} }}",
                self.ncl(),
                self.dc(),
                self.elt()
            )
        }
    }
    #[doc = "FDCAN TUR Numerator Actual Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Turna(pub u32);
    impl Turna {
        #[doc = "Numerator Actual Value"]
        #[inline(always)]
        pub const fn nav(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Numerator Actual Value"]
        #[inline(always)]
        pub fn set_nav(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for Turna {
        #[inline(always)]
        fn default() -> Turna {
            Turna(0)
        }
    }
    impl core::fmt::Debug for Turna {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Turna").field("nav", &self.nav()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Turna {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Turna {{ nav: {=u32:?} }}", self.nav())
        }
    }
    #[doc = "FDCAN Tx Buffer Add Request Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbar(pub u32);
    impl Txbar {
        #[doc = "Add Request"]
        #[inline(always)]
        pub const fn ar(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Add Request"]
        #[inline(always)]
        pub fn set_ar(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("ar[3]", &self.ar(3usize))
                .field("ar[4]", &self.ar(4usize))
                .field("ar[5]", &self.ar(5usize))
                .field("ar[6]", &self.ar(6usize))
                .field("ar[7]", &self.ar(7usize))
                .field("ar[8]", &self.ar(8usize))
                .field("ar[9]", &self.ar(9usize))
                .field("ar[10]", &self.ar(10usize))
                .field("ar[11]", &self.ar(11usize))
                .field("ar[12]", &self.ar(12usize))
                .field("ar[13]", &self.ar(13usize))
                .field("ar[14]", &self.ar(14usize))
                .field("ar[15]", &self.ar(15usize))
                .field("ar[16]", &self.ar(16usize))
                .field("ar[17]", &self.ar(17usize))
                .field("ar[18]", &self.ar(18usize))
                .field("ar[19]", &self.ar(19usize))
                .field("ar[20]", &self.ar(20usize))
                .field("ar[21]", &self.ar(21usize))
                .field("ar[22]", &self.ar(22usize))
                .field("ar[23]", &self.ar(23usize))
                .field("ar[24]", &self.ar(24usize))
                .field("ar[25]", &self.ar(25usize))
                .field("ar[26]", &self.ar(26usize))
                .field("ar[27]", &self.ar(27usize))
                .field("ar[28]", &self.ar(28usize))
                .field("ar[29]", &self.ar(29usize))
                .field("ar[30]", &self.ar(30usize))
                .field("ar[31]", &self.ar(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbar {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbar {{ ar[0]: {=bool:?}, ar[1]: {=bool:?}, ar[2]: {=bool:?}, ar[3]: {=bool:?}, ar[4]: {=bool:?}, ar[5]: {=bool:?}, ar[6]: {=bool:?}, ar[7]: {=bool:?}, ar[8]: {=bool:?}, ar[9]: {=bool:?}, ar[10]: {=bool:?}, ar[11]: {=bool:?}, ar[12]: {=bool:?}, ar[13]: {=bool:?}, ar[14]: {=bool:?}, ar[15]: {=bool:?}, ar[16]: {=bool:?}, ar[17]: {=bool:?}, ar[18]: {=bool:?}, ar[19]: {=bool:?}, ar[20]: {=bool:?}, ar[21]: {=bool:?}, ar[22]: {=bool:?}, ar[23]: {=bool:?}, ar[24]: {=bool:?}, ar[25]: {=bool:?}, ar[26]: {=bool:?}, ar[27]: {=bool:?}, ar[28]: {=bool:?}, ar[29]: {=bool:?}, ar[30]: {=bool:?}, ar[31]: {=bool:?} }}" , self . ar (0usize) , self . ar (1usize) , self . ar (2usize) , self . ar (3usize) , self . ar (4usize) , self . ar (5usize) , self . ar (6usize) , self . ar (7usize) , self . ar (8usize) , self . ar (9usize) , self . ar (10usize) , self . ar (11usize) , self . ar (12usize) , self . ar (13usize) , self . ar (14usize) , self . ar (15usize) , self . ar (16usize) , self . ar (17usize) , self . ar (18usize) , self . ar (19usize) , self . ar (20usize) , self . ar (21usize) , self . ar (22usize) , self . ar (23usize) , self . ar (24usize) , self . ar (25usize) , self . ar (26usize) , self . ar (27usize) , self . ar (28usize) , self . ar (29usize) , self . ar (30usize) , self . ar (31usize))
        }
    }
    #[doc = "FDCAN Tx Buffer Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbc(pub u32);
    impl Txbc {
        #[doc = "Tx Buffers Start Address"]
        #[inline(always)]
        pub const fn tbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Tx Buffers Start Address"]
        #[inline(always)]
        pub fn set_tbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Number of Dedicated Transmit Buffers"]
        #[inline(always)]
        pub const fn ndtb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of Dedicated Transmit Buffers"]
        #[inline(always)]
        pub fn set_ndtb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Transmit FIFO/Queue Size"]
        #[inline(always)]
        pub const fn tfqs(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Transmit FIFO/Queue Size"]
        #[inline(always)]
        pub fn set_tfqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Tx FIFO/Queue Mode"]
        #[inline(always)]
        pub const fn tfqm(&self) -> super::vals::Tfqm {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Tfqm::from_bits(val as u8)
        }
        #[doc = "Tx FIFO/Queue Mode"]
        #[inline(always)]
        pub fn set_tfqm(&mut self, val: super::vals::Tfqm) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
            f.debug_struct("Txbc")
                .field("tbsa", &self.tbsa())
                .field("ndtb", &self.ndtb())
                .field("tfqs", &self.tfqs())
                .field("tfqm", &self.tfqm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbc {{ tbsa: {=u16:?}, ndtb: {=u8:?}, tfqs: {=u8:?}, tfqm: {:?} }}",
                self.tbsa(),
                self.ndtb(),
                self.tfqs(),
                self.tfqm()
            )
        }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcf(pub u32);
    impl Txbcf {
        #[doc = "Cancellation Finished"]
        #[inline(always)]
        pub const fn cf(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation Finished"]
        #[inline(always)]
        pub fn set_cf(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("cf[3]", &self.cf(3usize))
                .field("cf[4]", &self.cf(4usize))
                .field("cf[5]", &self.cf(5usize))
                .field("cf[6]", &self.cf(6usize))
                .field("cf[7]", &self.cf(7usize))
                .field("cf[8]", &self.cf(8usize))
                .field("cf[9]", &self.cf(9usize))
                .field("cf[10]", &self.cf(10usize))
                .field("cf[11]", &self.cf(11usize))
                .field("cf[12]", &self.cf(12usize))
                .field("cf[13]", &self.cf(13usize))
                .field("cf[14]", &self.cf(14usize))
                .field("cf[15]", &self.cf(15usize))
                .field("cf[16]", &self.cf(16usize))
                .field("cf[17]", &self.cf(17usize))
                .field("cf[18]", &self.cf(18usize))
                .field("cf[19]", &self.cf(19usize))
                .field("cf[20]", &self.cf(20usize))
                .field("cf[21]", &self.cf(21usize))
                .field("cf[22]", &self.cf(22usize))
                .field("cf[23]", &self.cf(23usize))
                .field("cf[24]", &self.cf(24usize))
                .field("cf[25]", &self.cf(25usize))
                .field("cf[26]", &self.cf(26usize))
                .field("cf[27]", &self.cf(27usize))
                .field("cf[28]", &self.cf(28usize))
                .field("cf[29]", &self.cf(29usize))
                .field("cf[30]", &self.cf(30usize))
                .field("cf[31]", &self.cf(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcf {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbcf {{ cf[0]: {=bool:?}, cf[1]: {=bool:?}, cf[2]: {=bool:?}, cf[3]: {=bool:?}, cf[4]: {=bool:?}, cf[5]: {=bool:?}, cf[6]: {=bool:?}, cf[7]: {=bool:?}, cf[8]: {=bool:?}, cf[9]: {=bool:?}, cf[10]: {=bool:?}, cf[11]: {=bool:?}, cf[12]: {=bool:?}, cf[13]: {=bool:?}, cf[14]: {=bool:?}, cf[15]: {=bool:?}, cf[16]: {=bool:?}, cf[17]: {=bool:?}, cf[18]: {=bool:?}, cf[19]: {=bool:?}, cf[20]: {=bool:?}, cf[21]: {=bool:?}, cf[22]: {=bool:?}, cf[23]: {=bool:?}, cf[24]: {=bool:?}, cf[25]: {=bool:?}, cf[26]: {=bool:?}, cf[27]: {=bool:?}, cf[28]: {=bool:?}, cf[29]: {=bool:?}, cf[30]: {=bool:?}, cf[31]: {=bool:?} }}" , self . cf (0usize) , self . cf (1usize) , self . cf (2usize) , self . cf (3usize) , self . cf (4usize) , self . cf (5usize) , self . cf (6usize) , self . cf (7usize) , self . cf (8usize) , self . cf (9usize) , self . cf (10usize) , self . cf (11usize) , self . cf (12usize) , self . cf (13usize) , self . cf (14usize) , self . cf (15usize) , self . cf (16usize) , self . cf (17usize) , self . cf (18usize) , self . cf (19usize) , self . cf (20usize) , self . cf (21usize) , self . cf (22usize) , self . cf (23usize) , self . cf (24usize) , self . cf (25usize) , self . cf (26usize) , self . cf (27usize) , self . cf (28usize) , self . cf (29usize) , self . cf (30usize) , self . cf (31usize))
        }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcie(pub u32);
    impl Txbcie {
        #[doc = "Cancellation Finished Interrupt Enable"]
        #[inline(always)]
        pub const fn cf(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation Finished Interrupt Enable"]
        #[inline(always)]
        pub fn set_cf(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("cf[0]", &self.cf(0usize))
                .field("cf[1]", &self.cf(1usize))
                .field("cf[2]", &self.cf(2usize))
                .field("cf[3]", &self.cf(3usize))
                .field("cf[4]", &self.cf(4usize))
                .field("cf[5]", &self.cf(5usize))
                .field("cf[6]", &self.cf(6usize))
                .field("cf[7]", &self.cf(7usize))
                .field("cf[8]", &self.cf(8usize))
                .field("cf[9]", &self.cf(9usize))
                .field("cf[10]", &self.cf(10usize))
                .field("cf[11]", &self.cf(11usize))
                .field("cf[12]", &self.cf(12usize))
                .field("cf[13]", &self.cf(13usize))
                .field("cf[14]", &self.cf(14usize))
                .field("cf[15]", &self.cf(15usize))
                .field("cf[16]", &self.cf(16usize))
                .field("cf[17]", &self.cf(17usize))
                .field("cf[18]", &self.cf(18usize))
                .field("cf[19]", &self.cf(19usize))
                .field("cf[20]", &self.cf(20usize))
                .field("cf[21]", &self.cf(21usize))
                .field("cf[22]", &self.cf(22usize))
                .field("cf[23]", &self.cf(23usize))
                .field("cf[24]", &self.cf(24usize))
                .field("cf[25]", &self.cf(25usize))
                .field("cf[26]", &self.cf(26usize))
                .field("cf[27]", &self.cf(27usize))
                .field("cf[28]", &self.cf(28usize))
                .field("cf[29]", &self.cf(29usize))
                .field("cf[30]", &self.cf(30usize))
                .field("cf[31]", &self.cf(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcie {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbcie {{ cf[0]: {=bool:?}, cf[1]: {=bool:?}, cf[2]: {=bool:?}, cf[3]: {=bool:?}, cf[4]: {=bool:?}, cf[5]: {=bool:?}, cf[6]: {=bool:?}, cf[7]: {=bool:?}, cf[8]: {=bool:?}, cf[9]: {=bool:?}, cf[10]: {=bool:?}, cf[11]: {=bool:?}, cf[12]: {=bool:?}, cf[13]: {=bool:?}, cf[14]: {=bool:?}, cf[15]: {=bool:?}, cf[16]: {=bool:?}, cf[17]: {=bool:?}, cf[18]: {=bool:?}, cf[19]: {=bool:?}, cf[20]: {=bool:?}, cf[21]: {=bool:?}, cf[22]: {=bool:?}, cf[23]: {=bool:?}, cf[24]: {=bool:?}, cf[25]: {=bool:?}, cf[26]: {=bool:?}, cf[27]: {=bool:?}, cf[28]: {=bool:?}, cf[29]: {=bool:?}, cf[30]: {=bool:?}, cf[31]: {=bool:?} }}" , self . cf (0usize) , self . cf (1usize) , self . cf (2usize) , self . cf (3usize) , self . cf (4usize) , self . cf (5usize) , self . cf (6usize) , self . cf (7usize) , self . cf (8usize) , self . cf (9usize) , self . cf (10usize) , self . cf (11usize) , self . cf (12usize) , self . cf (13usize) , self . cf (14usize) , self . cf (15usize) , self . cf (16usize) , self . cf (17usize) , self . cf (18usize) , self . cf (19usize) , self . cf (20usize) , self . cf (21usize) , self . cf (22usize) , self . cf (23usize) , self . cf (24usize) , self . cf (25usize) , self . cf (26usize) , self . cf (27usize) , self . cf (28usize) , self . cf (29usize) , self . cf (30usize) , self . cf (31usize))
        }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Request Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcr(pub u32);
    impl Txbcr {
        #[doc = "Cancellation Request"]
        #[inline(always)]
        pub const fn cr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation Request"]
        #[inline(always)]
        pub fn set_cr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("cr[3]", &self.cr(3usize))
                .field("cr[4]", &self.cr(4usize))
                .field("cr[5]", &self.cr(5usize))
                .field("cr[6]", &self.cr(6usize))
                .field("cr[7]", &self.cr(7usize))
                .field("cr[8]", &self.cr(8usize))
                .field("cr[9]", &self.cr(9usize))
                .field("cr[10]", &self.cr(10usize))
                .field("cr[11]", &self.cr(11usize))
                .field("cr[12]", &self.cr(12usize))
                .field("cr[13]", &self.cr(13usize))
                .field("cr[14]", &self.cr(14usize))
                .field("cr[15]", &self.cr(15usize))
                .field("cr[16]", &self.cr(16usize))
                .field("cr[17]", &self.cr(17usize))
                .field("cr[18]", &self.cr(18usize))
                .field("cr[19]", &self.cr(19usize))
                .field("cr[20]", &self.cr(20usize))
                .field("cr[21]", &self.cr(21usize))
                .field("cr[22]", &self.cr(22usize))
                .field("cr[23]", &self.cr(23usize))
                .field("cr[24]", &self.cr(24usize))
                .field("cr[25]", &self.cr(25usize))
                .field("cr[26]", &self.cr(26usize))
                .field("cr[27]", &self.cr(27usize))
                .field("cr[28]", &self.cr(28usize))
                .field("cr[29]", &self.cr(29usize))
                .field("cr[30]", &self.cr(30usize))
                .field("cr[31]", &self.cr(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbcr {{ cr[0]: {=bool:?}, cr[1]: {=bool:?}, cr[2]: {=bool:?}, cr[3]: {=bool:?}, cr[4]: {=bool:?}, cr[5]: {=bool:?}, cr[6]: {=bool:?}, cr[7]: {=bool:?}, cr[8]: {=bool:?}, cr[9]: {=bool:?}, cr[10]: {=bool:?}, cr[11]: {=bool:?}, cr[12]: {=bool:?}, cr[13]: {=bool:?}, cr[14]: {=bool:?}, cr[15]: {=bool:?}, cr[16]: {=bool:?}, cr[17]: {=bool:?}, cr[18]: {=bool:?}, cr[19]: {=bool:?}, cr[20]: {=bool:?}, cr[21]: {=bool:?}, cr[22]: {=bool:?}, cr[23]: {=bool:?}, cr[24]: {=bool:?}, cr[25]: {=bool:?}, cr[26]: {=bool:?}, cr[27]: {=bool:?}, cr[28]: {=bool:?}, cr[29]: {=bool:?}, cr[30]: {=bool:?}, cr[31]: {=bool:?} }}" , self . cr (0usize) , self . cr (1usize) , self . cr (2usize) , self . cr (3usize) , self . cr (4usize) , self . cr (5usize) , self . cr (6usize) , self . cr (7usize) , self . cr (8usize) , self . cr (9usize) , self . cr (10usize) , self . cr (11usize) , self . cr (12usize) , self . cr (13usize) , self . cr (14usize) , self . cr (15usize) , self . cr (16usize) , self . cr (17usize) , self . cr (18usize) , self . cr (19usize) , self . cr (20usize) , self . cr (21usize) , self . cr (22usize) , self . cr (23usize) , self . cr (24usize) , self . cr (25usize) , self . cr (26usize) , self . cr (27usize) , self . cr (28usize) , self . cr (29usize) , self . cr (30usize) , self . cr (31usize))
        }
    }
    #[doc = "FDCAN Tx Buffer Request Pending Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbrp(pub u32);
    impl Txbrp {
        #[doc = "Transmission Request Pending"]
        #[inline(always)]
        pub const fn trp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission Request Pending"]
        #[inline(always)]
        pub fn set_trp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("trp[3]", &self.trp(3usize))
                .field("trp[4]", &self.trp(4usize))
                .field("trp[5]", &self.trp(5usize))
                .field("trp[6]", &self.trp(6usize))
                .field("trp[7]", &self.trp(7usize))
                .field("trp[8]", &self.trp(8usize))
                .field("trp[9]", &self.trp(9usize))
                .field("trp[10]", &self.trp(10usize))
                .field("trp[11]", &self.trp(11usize))
                .field("trp[12]", &self.trp(12usize))
                .field("trp[13]", &self.trp(13usize))
                .field("trp[14]", &self.trp(14usize))
                .field("trp[15]", &self.trp(15usize))
                .field("trp[16]", &self.trp(16usize))
                .field("trp[17]", &self.trp(17usize))
                .field("trp[18]", &self.trp(18usize))
                .field("trp[19]", &self.trp(19usize))
                .field("trp[20]", &self.trp(20usize))
                .field("trp[21]", &self.trp(21usize))
                .field("trp[22]", &self.trp(22usize))
                .field("trp[23]", &self.trp(23usize))
                .field("trp[24]", &self.trp(24usize))
                .field("trp[25]", &self.trp(25usize))
                .field("trp[26]", &self.trp(26usize))
                .field("trp[27]", &self.trp(27usize))
                .field("trp[28]", &self.trp(28usize))
                .field("trp[29]", &self.trp(29usize))
                .field("trp[30]", &self.trp(30usize))
                .field("trp[31]", &self.trp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbrp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbrp {{ trp[0]: {=bool:?}, trp[1]: {=bool:?}, trp[2]: {=bool:?}, trp[3]: {=bool:?}, trp[4]: {=bool:?}, trp[5]: {=bool:?}, trp[6]: {=bool:?}, trp[7]: {=bool:?}, trp[8]: {=bool:?}, trp[9]: {=bool:?}, trp[10]: {=bool:?}, trp[11]: {=bool:?}, trp[12]: {=bool:?}, trp[13]: {=bool:?}, trp[14]: {=bool:?}, trp[15]: {=bool:?}, trp[16]: {=bool:?}, trp[17]: {=bool:?}, trp[18]: {=bool:?}, trp[19]: {=bool:?}, trp[20]: {=bool:?}, trp[21]: {=bool:?}, trp[22]: {=bool:?}, trp[23]: {=bool:?}, trp[24]: {=bool:?}, trp[25]: {=bool:?}, trp[26]: {=bool:?}, trp[27]: {=bool:?}, trp[28]: {=bool:?}, trp[29]: {=bool:?}, trp[30]: {=bool:?}, trp[31]: {=bool:?} }}" , self . trp (0usize) , self . trp (1usize) , self . trp (2usize) , self . trp (3usize) , self . trp (4usize) , self . trp (5usize) , self . trp (6usize) , self . trp (7usize) , self . trp (8usize) , self . trp (9usize) , self . trp (10usize) , self . trp (11usize) , self . trp (12usize) , self . trp (13usize) , self . trp (14usize) , self . trp (15usize) , self . trp (16usize) , self . trp (17usize) , self . trp (18usize) , self . trp (19usize) , self . trp (20usize) , self . trp (21usize) , self . trp (22usize) , self . trp (23usize) , self . trp (24usize) , self . trp (25usize) , self . trp (26usize) , self . trp (27usize) , self . trp (28usize) , self . trp (29usize) , self . trp (30usize) , self . trp (31usize))
        }
    }
    #[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbtie(pub u32);
    impl Txbtie {
        #[doc = "Transmission Interrupt Enable"]
        #[inline(always)]
        pub const fn tie(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission Interrupt Enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("tie[3]", &self.tie(3usize))
                .field("tie[4]", &self.tie(4usize))
                .field("tie[5]", &self.tie(5usize))
                .field("tie[6]", &self.tie(6usize))
                .field("tie[7]", &self.tie(7usize))
                .field("tie[8]", &self.tie(8usize))
                .field("tie[9]", &self.tie(9usize))
                .field("tie[10]", &self.tie(10usize))
                .field("tie[11]", &self.tie(11usize))
                .field("tie[12]", &self.tie(12usize))
                .field("tie[13]", &self.tie(13usize))
                .field("tie[14]", &self.tie(14usize))
                .field("tie[15]", &self.tie(15usize))
                .field("tie[16]", &self.tie(16usize))
                .field("tie[17]", &self.tie(17usize))
                .field("tie[18]", &self.tie(18usize))
                .field("tie[19]", &self.tie(19usize))
                .field("tie[20]", &self.tie(20usize))
                .field("tie[21]", &self.tie(21usize))
                .field("tie[22]", &self.tie(22usize))
                .field("tie[23]", &self.tie(23usize))
                .field("tie[24]", &self.tie(24usize))
                .field("tie[25]", &self.tie(25usize))
                .field("tie[26]", &self.tie(26usize))
                .field("tie[27]", &self.tie(27usize))
                .field("tie[28]", &self.tie(28usize))
                .field("tie[29]", &self.tie(29usize))
                .field("tie[30]", &self.tie(30usize))
                .field("tie[31]", &self.tie(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbtie {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbtie {{ tie[0]: {=bool:?}, tie[1]: {=bool:?}, tie[2]: {=bool:?}, tie[3]: {=bool:?}, tie[4]: {=bool:?}, tie[5]: {=bool:?}, tie[6]: {=bool:?}, tie[7]: {=bool:?}, tie[8]: {=bool:?}, tie[9]: {=bool:?}, tie[10]: {=bool:?}, tie[11]: {=bool:?}, tie[12]: {=bool:?}, tie[13]: {=bool:?}, tie[14]: {=bool:?}, tie[15]: {=bool:?}, tie[16]: {=bool:?}, tie[17]: {=bool:?}, tie[18]: {=bool:?}, tie[19]: {=bool:?}, tie[20]: {=bool:?}, tie[21]: {=bool:?}, tie[22]: {=bool:?}, tie[23]: {=bool:?}, tie[24]: {=bool:?}, tie[25]: {=bool:?}, tie[26]: {=bool:?}, tie[27]: {=bool:?}, tie[28]: {=bool:?}, tie[29]: {=bool:?}, tie[30]: {=bool:?}, tie[31]: {=bool:?} }}" , self . tie (0usize) , self . tie (1usize) , self . tie (2usize) , self . tie (3usize) , self . tie (4usize) , self . tie (5usize) , self . tie (6usize) , self . tie (7usize) , self . tie (8usize) , self . tie (9usize) , self . tie (10usize) , self . tie (11usize) , self . tie (12usize) , self . tie (13usize) , self . tie (14usize) , self . tie (15usize) , self . tie (16usize) , self . tie (17usize) , self . tie (18usize) , self . tie (19usize) , self . tie (20usize) , self . tie (21usize) , self . tie (22usize) , self . tie (23usize) , self . tie (24usize) , self . tie (25usize) , self . tie (26usize) , self . tie (27usize) , self . tie (28usize) , self . tie (29usize) , self . tie (30usize) , self . tie (31usize))
        }
    }
    #[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbto(pub u32);
    impl Txbto {
        #[doc = "Transmission Occurred"]
        #[inline(always)]
        pub const fn to(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission Occurred"]
        #[inline(always)]
        pub fn set_to(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("to[3]", &self.to(3usize))
                .field("to[4]", &self.to(4usize))
                .field("to[5]", &self.to(5usize))
                .field("to[6]", &self.to(6usize))
                .field("to[7]", &self.to(7usize))
                .field("to[8]", &self.to(8usize))
                .field("to[9]", &self.to(9usize))
                .field("to[10]", &self.to(10usize))
                .field("to[11]", &self.to(11usize))
                .field("to[12]", &self.to(12usize))
                .field("to[13]", &self.to(13usize))
                .field("to[14]", &self.to(14usize))
                .field("to[15]", &self.to(15usize))
                .field("to[16]", &self.to(16usize))
                .field("to[17]", &self.to(17usize))
                .field("to[18]", &self.to(18usize))
                .field("to[19]", &self.to(19usize))
                .field("to[20]", &self.to(20usize))
                .field("to[21]", &self.to(21usize))
                .field("to[22]", &self.to(22usize))
                .field("to[23]", &self.to(23usize))
                .field("to[24]", &self.to(24usize))
                .field("to[25]", &self.to(25usize))
                .field("to[26]", &self.to(26usize))
                .field("to[27]", &self.to(27usize))
                .field("to[28]", &self.to(28usize))
                .field("to[29]", &self.to(29usize))
                .field("to[30]", &self.to(30usize))
                .field("to[31]", &self.to(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbto {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txbto {{ to[0]: {=bool:?}, to[1]: {=bool:?}, to[2]: {=bool:?}, to[3]: {=bool:?}, to[4]: {=bool:?}, to[5]: {=bool:?}, to[6]: {=bool:?}, to[7]: {=bool:?}, to[8]: {=bool:?}, to[9]: {=bool:?}, to[10]: {=bool:?}, to[11]: {=bool:?}, to[12]: {=bool:?}, to[13]: {=bool:?}, to[14]: {=bool:?}, to[15]: {=bool:?}, to[16]: {=bool:?}, to[17]: {=bool:?}, to[18]: {=bool:?}, to[19]: {=bool:?}, to[20]: {=bool:?}, to[21]: {=bool:?}, to[22]: {=bool:?}, to[23]: {=bool:?}, to[24]: {=bool:?}, to[25]: {=bool:?}, to[26]: {=bool:?}, to[27]: {=bool:?}, to[28]: {=bool:?}, to[29]: {=bool:?}, to[30]: {=bool:?}, to[31]: {=bool:?} }}" , self . to (0usize) , self . to (1usize) , self . to (2usize) , self . to (3usize) , self . to (4usize) , self . to (5usize) , self . to (6usize) , self . to (7usize) , self . to (8usize) , self . to (9usize) , self . to (10usize) , self . to (11usize) , self . to (12usize) , self . to (13usize) , self . to (14usize) , self . to (15usize) , self . to (16usize) , self . to (17usize) , self . to (18usize) , self . to (19usize) , self . to (20usize) , self . to (21usize) , self . to (22usize) , self . to (23usize) , self . to (24usize) , self . to (25usize) , self . to (26usize) , self . to (27usize) , self . to (28usize) , self . to (29usize) , self . to (30usize) , self . to (31usize))
        }
    }
    #[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefa(pub u32);
    impl Txefa {
        #[doc = "Event FIFO Acknowledge Index"]
        #[inline(always)]
        pub const fn efai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Acknowledge Index"]
        #[inline(always)]
        pub fn set_efai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
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
    #[doc = "FDCAN Tx Event FIFO Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefc(pub u32);
    impl Txefc {
        #[doc = "Event FIFO Start Address"]
        #[inline(always)]
        pub const fn efsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Event FIFO Start Address"]
        #[inline(always)]
        pub fn set_efsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Event FIFO Size"]
        #[inline(always)]
        pub const fn efs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Size"]
        #[inline(always)]
        pub fn set_efs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Event FIFO Watermark"]
        #[inline(always)]
        pub const fn efwm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Watermark"]
        #[inline(always)]
        pub fn set_efwm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Txefc {
        #[inline(always)]
        fn default() -> Txefc {
            Txefc(0)
        }
    }
    impl core::fmt::Debug for Txefc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txefc")
                .field("efsa", &self.efsa())
                .field("efs", &self.efs())
                .field("efwm", &self.efwm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txefc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txefc {{ efsa: {=u16:?}, efs: {=u8:?}, efwm: {=u8:?} }}",
                self.efsa(),
                self.efs(),
                self.efwm()
            )
        }
    }
    #[doc = "FDCAN Tx Event FIFO Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefs(pub u32);
    impl Txefs {
        #[doc = "Event FIFO Fill Level"]
        #[inline(always)]
        pub const fn effl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Fill Level"]
        #[inline(always)]
        pub fn set_effl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Event FIFO Get Index"]
        #[inline(always)]
        pub const fn efgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Get Index"]
        #[inline(always)]
        pub fn set_efgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Event FIFO put index"]
        #[inline(always)]
        pub const fn efpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO put index"]
        #[inline(always)]
        pub fn set_efpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Event FIFO Full"]
        #[inline(always)]
        pub const fn eff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Event FIFO Full"]
        #[inline(always)]
        pub fn set_eff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost"]
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
    #[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txesc(pub u32);
    impl Txesc {
        #[doc = "Tx Buffer Data Field Size"]
        #[inline(always)]
        pub const fn tbds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tx Buffer Data Field Size"]
        #[inline(always)]
        pub fn set_tbds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Txesc {
        #[inline(always)]
        fn default() -> Txesc {
            Txesc(0)
        }
    }
    impl core::fmt::Debug for Txesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txesc").field("tbds", &self.tbds()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txesc {{ tbds: {=u8:?} }}", self.tbds())
        }
    }
    #[doc = "FDCAN Tx FIFO/Queue Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txfqs(pub u32);
    impl Txfqs {
        #[doc = "Tx FIFO Free Level"]
        #[inline(always)]
        pub const fn tffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Tx FIFO Free Level"]
        #[inline(always)]
        pub fn set_tffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "TFGI"]
        #[inline(always)]
        pub const fn tfgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "TFGI"]
        #[inline(always)]
        pub fn set_tfgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Tx FIFO/Queue Put Index"]
        #[inline(always)]
        pub const fn tfqpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Tx FIFO/Queue Put Index"]
        #[inline(always)]
        pub fn set_tfqpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Tx FIFO/Queue Full"]
        #[inline(always)]
        pub const fn tfqf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO/Queue Full"]
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
    #[doc = "FDCAN Extended ID and Mask Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidam(pub u32);
    impl Xidam {
        #[doc = "Extended ID Mask"]
        #[inline(always)]
        pub const fn eidm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Extended ID Mask"]
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
    #[doc = "FDCAN Extended ID Filter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidfc(pub u32);
    impl Xidfc {
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub const fn flesa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub fn set_flesa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "List Size Extended"]
        #[inline(always)]
        pub const fn lse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "List Size Extended"]
        #[inline(always)]
        pub fn set_lse(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Xidfc {
        #[inline(always)]
        fn default() -> Xidfc {
            Xidfc(0)
        }
    }
    impl core::fmt::Debug for Xidfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xidfc")
                .field("flesa", &self.flesa())
                .field("lse", &self.lse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xidfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Xidfc {{ flesa: {=u16:?}, lse: {=u8:?} }}", self.flesa(), self.lse())
        }
    }
}
pub mod vals {
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
}
