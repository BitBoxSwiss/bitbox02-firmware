#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Low-power Universal synchronous asynchronous receiver transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart {
    ptr: *mut u8,
}
unsafe impl Send for Lpuart {}
unsafe impl Sync for Lpuart {}
impl Lpuart {
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
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Request register"]
    #[inline(always)]
    pub const fn rqr(self) -> crate::common::Reg<regs::Rqr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Interrupt & status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receive data register"]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Dr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart {
    ptr: *mut u8,
}
unsafe impl Send for Usart {}
unsafe impl Sync for Usart {}
impl Usart {
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
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Guard time and prescaler register"]
    #[inline(always)]
    pub const fn gtpr(self) -> crate::common::Reg<regs::Gtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Receiver timeout register"]
    #[inline(always)]
    pub const fn rtor(self) -> crate::common::Reg<regs::Rtor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Request register"]
    #[inline(always)]
    pub const fn rqr(self) -> crate::common::Reg<regs::Rqr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Interrupt & status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receive data register"]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Dr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "Baud rate register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "USARTDIV"]
        #[inline(always)]
        pub const fn brr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "USARTDIV"]
        #[inline(always)]
        pub fn set_brr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr").field("brr", &self.brr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brr {{ brr: {=u16:?} }}", self.brr())
        }
    }
    #[doc = "Control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "USART enable"]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable"]
        #[inline(always)]
        pub fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART enable in Stop mode"]
        #[inline(always)]
        pub const fn uesm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable in Stop mode"]
        #[inline(always)]
        pub fn set_uesm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IDLE interrupt enable"]
        #[inline(always)]
        pub const fn idleie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE interrupt enable"]
        #[inline(always)]
        pub fn set_idleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TXE interrupt enable"]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TXE interrupt enable"]
        #[inline(always)]
        pub fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PE interrupt enable"]
        #[inline(always)]
        pub const fn peie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PE interrupt enable"]
        #[inline(always)]
        pub fn set_peie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Parity selection"]
        #[inline(always)]
        pub const fn ps(&self) -> super::vals::Ps {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Ps::from_bits(val as u8)
        }
        #[doc = "Parity selection"]
        #[inline(always)]
        pub fn set_ps(&mut self, val: super::vals::Ps) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Parity control enable"]
        #[inline(always)]
        pub const fn pce(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Parity control enable"]
        #[inline(always)]
        pub fn set_pce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver wakeup method"]
        #[inline(always)]
        pub const fn wake(&self) -> super::vals::Wake {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wake::from_bits(val as u8)
        }
        #[doc = "Receiver wakeup method"]
        #[inline(always)]
        pub fn set_wake(&mut self, val: super::vals::Wake) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub const fn m0(&self) -> super::vals::M0 {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::M0::from_bits(val as u8)
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub fn set_m0(&mut self, val: super::vals::M0) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Mute mode enable"]
        #[inline(always)]
        pub const fn mme(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Mute mode enable"]
        #[inline(always)]
        pub fn set_mme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Character match interrupt enable"]
        #[inline(always)]
        pub const fn cmie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Character match interrupt enable"]
        #[inline(always)]
        pub fn set_cmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Oversampling mode"]
        #[inline(always)]
        pub const fn over8(&self) -> super::vals::Over8 {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Over8::from_bits(val as u8)
        }
        #[doc = "Oversampling mode"]
        #[inline(always)]
        pub fn set_over8(&mut self, val: super::vals::Over8) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Driver Enable deassertion time"]
        #[inline(always)]
        pub const fn dedt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Driver Enable deassertion time"]
        #[inline(always)]
        pub fn set_dedt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Driver Enable assertion time"]
        #[inline(always)]
        pub const fn deat(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "Driver Enable assertion time"]
        #[inline(always)]
        pub fn set_deat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "Receiver timeout interrupt enable"]
        #[inline(always)]
        pub const fn rtoie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout interrupt enable"]
        #[inline(always)]
        pub fn set_rtoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "End of Block interrupt enable"]
        #[inline(always)]
        pub const fn eobie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "End of Block interrupt enable"]
        #[inline(always)]
        pub fn set_eobie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub const fn m1(&self) -> super::vals::M1 {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::M1::from_bits(val as u8)
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub fn set_m1(&mut self, val: super::vals::M1) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
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
                .field("ue", &self.ue())
                .field("uesm", &self.uesm())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("idleie", &self.idleie())
                .field("rxneie", &self.rxneie())
                .field("tcie", &self.tcie())
                .field("txeie", &self.txeie())
                .field("peie", &self.peie())
                .field("ps", &self.ps())
                .field("pce", &self.pce())
                .field("wake", &self.wake())
                .field("m0", &self.m0())
                .field("mme", &self.mme())
                .field("cmie", &self.cmie())
                .field("over8", &self.over8())
                .field("dedt", &self.dedt())
                .field("deat", &self.deat())
                .field("rtoie", &self.rtoie())
                .field("eobie", &self.eobie())
                .field("m1", &self.m1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ ue: {=bool:?}, uesm: {=bool:?}, re: {=bool:?}, te: {=bool:?}, idleie: {=bool:?}, rxneie: {=bool:?}, tcie: {=bool:?}, txeie: {=bool:?}, peie: {=bool:?}, ps: {:?}, pce: {=bool:?}, wake: {:?}, m0: {:?}, mme: {=bool:?}, cmie: {=bool:?}, over8: {:?}, dedt: {=u8:?}, deat: {=u8:?}, rtoie: {=bool:?}, eobie: {=bool:?}, m1: {:?} }}" , self . ue () , self . uesm () , self . re () , self . te () , self . idleie () , self . rxneie () , self . tcie () , self . txeie () , self . peie () , self . ps () , self . pce () , self . wake () , self . m0 () , self . mme () , self . cmie () , self . over8 () , self . dedt () , self . deat () , self . rtoie () , self . eobie () , self . m1 ())
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "7-bit Address Detection/4-bit Address Detection"]
        #[inline(always)]
        pub const fn addm(&self) -> super::vals::Addm {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Addm::from_bits(val as u8)
        }
        #[doc = "7-bit Address Detection/4-bit Address Detection"]
        #[inline(always)]
        pub fn set_addm(&mut self, val: super::vals::Addm) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub const fn lbdl(&self) -> super::vals::Lbdl {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Lbdl::from_bits(val as u8)
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub const fn lbdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub fn set_lbdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Last bit clock pulse"]
        #[inline(always)]
        pub const fn lbcl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Last bit clock pulse"]
        #[inline(always)]
        pub fn set_lbcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub const fn cpha(&self) -> super::vals::Cpha {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Cpha::from_bits(val as u8)
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub fn set_cpha(&mut self, val: super::vals::Cpha) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub const fn cpol(&self) -> super::vals::Cpol {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Cpol::from_bits(val as u8)
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub fn set_cpol(&mut self, val: super::vals::Cpol) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Clock enable"]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable"]
        #[inline(always)]
        pub fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::Stop {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stop::from_bits(val as u8)
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub fn set_stop(&mut self, val: super::vals::Stop) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub const fn linen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub fn set_linen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Swap TX/RX pins"]
        #[inline(always)]
        pub const fn swap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Swap TX/RX pins"]
        #[inline(always)]
        pub fn set_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RX pin active level inversion"]
        #[inline(always)]
        pub const fn rxinv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin active level inversion"]
        #[inline(always)]
        pub fn set_rxinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TX pin active level inversion"]
        #[inline(always)]
        pub const fn txinv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin active level inversion"]
        #[inline(always)]
        pub fn set_txinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Binary data inversion"]
        #[inline(always)]
        pub const fn datainv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Binary data inversion"]
        #[inline(always)]
        pub fn set_datainv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Most significant bit first"]
        #[inline(always)]
        pub const fn msbfirst(&self) -> super::vals::Msbfirst {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Msbfirst::from_bits(val as u8)
        }
        #[doc = "Most significant bit first"]
        #[inline(always)]
        pub fn set_msbfirst(&mut self, val: super::vals::Msbfirst) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Auto baud rate enable"]
        #[inline(always)]
        pub const fn abren(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate enable"]
        #[inline(always)]
        pub fn set_abren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Auto baud rate mode"]
        #[inline(always)]
        pub const fn abrmod(&self) -> super::vals::Abrmod {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Abrmod::from_bits(val as u8)
        }
        #[doc = "Auto baud rate mode"]
        #[inline(always)]
        pub fn set_abrmod(&mut self, val: super::vals::Abrmod) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "Receiver timeout enable"]
        #[inline(always)]
        pub const fn rtoen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout enable"]
        #[inline(always)]
        pub fn set_rtoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
                .field("addm", &self.addm())
                .field("lbdl", &self.lbdl())
                .field("lbdie", &self.lbdie())
                .field("lbcl", &self.lbcl())
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("clken", &self.clken())
                .field("stop", &self.stop())
                .field("linen", &self.linen())
                .field("swap", &self.swap())
                .field("rxinv", &self.rxinv())
                .field("txinv", &self.txinv())
                .field("datainv", &self.datainv())
                .field("msbfirst", &self.msbfirst())
                .field("abren", &self.abren())
                .field("abrmod", &self.abrmod())
                .field("rtoen", &self.rtoen())
                .field("add", &self.add())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ addm: {:?}, lbdl: {:?}, lbdie: {=bool:?}, lbcl: {=bool:?}, cpha: {:?}, cpol: {:?}, clken: {=bool:?}, stop: {:?}, linen: {=bool:?}, swap: {=bool:?}, rxinv: {=bool:?}, txinv: {=bool:?}, datainv: {=bool:?}, msbfirst: {:?}, abren: {=bool:?}, abrmod: {:?}, rtoen: {=bool:?}, add: {=u8:?} }}" , self . addm () , self . lbdl () , self . lbdie () , self . lbcl () , self . cpha () , self . cpol () , self . clken () , self . stop () , self . linen () , self . swap () , self . rxinv () , self . txinv () , self . datainv () , self . msbfirst () , self . abren () , self . abrmod () , self . rtoen () , self . add ())
        }
    }
    #[doc = "Control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub const fn irlp(&self) -> super::vals::Irlp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Irlp::from_bits(val as u8)
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub fn set_irlp(&mut self, val: super::vals::Irlp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Smartcard NACK enable"]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard NACK enable"]
        #[inline(always)]
        pub fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Smartcard mode enable"]
        #[inline(always)]
        pub const fn scen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard mode enable"]
        #[inline(always)]
        pub fn set_scen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub const fn dmar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub fn set_dmar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub const fn dmat(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub fn set_dmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTS enable"]
        #[inline(always)]
        pub const fn rtse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RTS enable"]
        #[inline(always)]
        pub fn set_rtse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS enable"]
        #[inline(always)]
        pub const fn ctse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS enable"]
        #[inline(always)]
        pub fn set_ctse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS interrupt enable"]
        #[inline(always)]
        pub const fn ctsie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt enable"]
        #[inline(always)]
        pub fn set_ctsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "One sample bit method enable"]
        #[inline(always)]
        pub const fn onebit(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "One sample bit method enable"]
        #[inline(always)]
        pub fn set_onebit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Overrun Disable"]
        #[inline(always)]
        pub const fn ovrdis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun Disable"]
        #[inline(always)]
        pub fn set_ovrdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA Disable on Reception Error"]
        #[inline(always)]
        pub const fn ddre(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Disable on Reception Error"]
        #[inline(always)]
        pub fn set_ddre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Driver enable mode"]
        #[inline(always)]
        pub const fn dem(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Driver enable mode"]
        #[inline(always)]
        pub fn set_dem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Driver enable polarity selection"]
        #[inline(always)]
        pub const fn dep(&self) -> super::vals::Dep {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dep::from_bits(val as u8)
        }
        #[doc = "Driver enable polarity selection"]
        #[inline(always)]
        pub fn set_dep(&mut self, val: super::vals::Dep) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Smartcard auto-retry count"]
        #[inline(always)]
        pub const fn scarcnt(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Smartcard auto-retry count"]
        #[inline(always)]
        pub fn set_scarcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Wakeup from Stop mode interrupt flag selection"]
        #[inline(always)]
        pub const fn wus(&self) -> super::vals::Wus {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Wus::from_bits(val as u8)
        }
        #[doc = "Wakeup from Stop mode interrupt flag selection"]
        #[inline(always)]
        pub fn set_wus(&mut self, val: super::vals::Wus) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Wakeup from Stop mode interrupt enable"]
        #[inline(always)]
        pub const fn wufie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode interrupt enable"]
        #[inline(always)]
        pub fn set_wufie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("eie", &self.eie())
                .field("iren", &self.iren())
                .field("irlp", &self.irlp())
                .field("hdsel", &self.hdsel())
                .field("nack", &self.nack())
                .field("scen", &self.scen())
                .field("dmar", &self.dmar())
                .field("dmat", &self.dmat())
                .field("rtse", &self.rtse())
                .field("ctse", &self.ctse())
                .field("ctsie", &self.ctsie())
                .field("onebit", &self.onebit())
                .field("ovrdis", &self.ovrdis())
                .field("ddre", &self.ddre())
                .field("dem", &self.dem())
                .field("dep", &self.dep())
                .field("scarcnt", &self.scarcnt())
                .field("wus", &self.wus())
                .field("wufie", &self.wufie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr3 {{ eie: {=bool:?}, iren: {=bool:?}, irlp: {:?}, hdsel: {=bool:?}, nack: {=bool:?}, scen: {=bool:?}, dmar: {=bool:?}, dmat: {=bool:?}, rtse: {=bool:?}, ctse: {=bool:?}, ctsie: {=bool:?}, onebit: {=bool:?}, ovrdis: {=bool:?}, ddre: {=bool:?}, dem: {=bool:?}, dep: {:?}, scarcnt: {=u8:?}, wus: {:?}, wufie: {=bool:?} }}" , self . eie () , self . iren () , self . irlp () , self . hdsel () , self . nack () , self . scen () , self . dmar () , self . dmat () , self . rtse () , self . ctse () , self . ctsie () , self . onebit () , self . ovrdis () , self . ddre () , self . dem () , self . dep () , self . scarcnt () , self . wus () , self . wufie ())
        }
    }
    #[doc = "Data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data value"]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Data value"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
            defmt::write!(f, "Dr {{ dr: {=u16:?} }}", self.dr())
        }
    }
    #[doc = "Guard time and prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtpr(pub u32);
    impl Gtpr {
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub const fn psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub fn set_psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Guard time value"]
        #[inline(always)]
        pub const fn gt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Guard time value"]
        #[inline(always)]
        pub fn set_gt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Gtpr {
        #[inline(always)]
        fn default() -> Gtpr {
            Gtpr(0)
        }
    }
    impl core::fmt::Debug for Gtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtpr")
                .field("psc", &self.psc())
                .field("gt", &self.gt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gtpr {{ psc: {=u8:?}, gt: {=u8:?} }}", self.psc(), self.gt())
        }
    }
    #[doc = "Interrupt flag clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Parity error clear flag"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error clear flag"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error clear flag"]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error clear flag"]
        #[inline(always)]
        pub fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise error clear flag"]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise error clear flag"]
        #[inline(always)]
        pub fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error clear flag"]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error clear flag"]
        #[inline(always)]
        pub fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Idle line detected clear flag"]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Idle line detected clear flag"]
        #[inline(always)]
        pub fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmission complete clear flag"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete clear flag"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LIN break detection clear flag"]
        #[inline(always)]
        pub const fn lbd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection clear flag"]
        #[inline(always)]
        pub fn set_lbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS clear flag"]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS clear flag"]
        #[inline(always)]
        pub fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receiver timeout clear flag"]
        #[inline(always)]
        pub const fn rtof(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout clear flag"]
        #[inline(always)]
        pub fn set_rtof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "End of block clear flag"]
        #[inline(always)]
        pub const fn eobf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of block clear flag"]
        #[inline(always)]
        pub fn set_eobf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Character match clear flag"]
        #[inline(always)]
        pub const fn cmf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Character match clear flag"]
        #[inline(always)]
        pub fn set_cmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Wakeup from Stop mode clear flag"]
        #[inline(always)]
        pub const fn wuf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode clear flag"]
        #[inline(always)]
        pub fn set_wuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("idle", &self.idle())
                .field("tc", &self.tc())
                .field("lbd", &self.lbd())
                .field("cts", &self.cts())
                .field("rtof", &self.rtof())
                .field("eobf", &self.eobf())
                .field("cmf", &self.cmf())
                .field("wuf", &self.wuf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ pe: {=bool:?}, fe: {=bool:?}, ne: {=bool:?}, ore: {=bool:?}, idle: {=bool:?}, tc: {=bool:?}, lbd: {=bool:?}, cts: {=bool:?}, rtof: {=bool:?}, eobf: {=bool:?}, cmf: {=bool:?}, wuf: {=bool:?} }}" , self . pe () , self . fe () , self . ne () , self . ore () , self . idle () , self . tc () , self . lbd () , self . cts () , self . rtof () , self . eobf () , self . cmf () , self . wuf ())
        }
    }
    #[doc = "Interrupt & status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Parity error"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise error flag"]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise error flag"]
        #[inline(always)]
        pub fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Idle line detected"]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Idle line detected"]
        #[inline(always)]
        pub fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmit data register empty"]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data register empty"]
        #[inline(always)]
        pub fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LIN break detection flag"]
        #[inline(always)]
        pub const fn lbd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection flag"]
        #[inline(always)]
        pub fn set_lbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS interrupt flag"]
        #[inline(always)]
        pub const fn ctsif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt flag"]
        #[inline(always)]
        pub fn set_ctsif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS flag"]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS flag"]
        #[inline(always)]
        pub fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub const fn rtof(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout"]
        #[inline(always)]
        pub fn set_rtof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "End of block flag"]
        #[inline(always)]
        pub const fn eobf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of block flag"]
        #[inline(always)]
        pub fn set_eobf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Auto baud rate error"]
        #[inline(always)]
        pub const fn abre(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate error"]
        #[inline(always)]
        pub fn set_abre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Auto baud rate flag"]
        #[inline(always)]
        pub const fn abrf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate flag"]
        #[inline(always)]
        pub fn set_abrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "character match flag"]
        #[inline(always)]
        pub const fn cmf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "character match flag"]
        #[inline(always)]
        pub fn set_cmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Send break flag"]
        #[inline(always)]
        pub const fn sbkf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Send break flag"]
        #[inline(always)]
        pub fn set_sbkf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receiver wakeup from Mute mode"]
        #[inline(always)]
        pub const fn rwu(&self) -> super::vals::Rwu {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Rwu::from_bits(val as u8)
        }
        #[doc = "Receiver wakeup from Mute mode"]
        #[inline(always)]
        pub fn set_rwu(&mut self, val: super::vals::Rwu) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Wakeup from Stop mode flag"]
        #[inline(always)]
        pub const fn wuf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode flag"]
        #[inline(always)]
        pub fn set_wuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit enable acknowledge flag"]
        #[inline(always)]
        pub const fn teack(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit enable acknowledge flag"]
        #[inline(always)]
        pub fn set_teack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Receive enable acknowledge flag"]
        #[inline(always)]
        pub const fn reack(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Receive enable acknowledge flag"]
        #[inline(always)]
        pub fn set_reack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("idle", &self.idle())
                .field("rxne", &self.rxne())
                .field("tc", &self.tc())
                .field("txe", &self.txe())
                .field("lbd", &self.lbd())
                .field("ctsif", &self.ctsif())
                .field("cts", &self.cts())
                .field("rtof", &self.rtof())
                .field("eobf", &self.eobf())
                .field("abre", &self.abre())
                .field("abrf", &self.abrf())
                .field("busy", &self.busy())
                .field("cmf", &self.cmf())
                .field("sbkf", &self.sbkf())
                .field("rwu", &self.rwu())
                .field("wuf", &self.wuf())
                .field("teack", &self.teack())
                .field("reack", &self.reack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ pe: {=bool:?}, fe: {=bool:?}, ne: {=bool:?}, ore: {=bool:?}, idle: {=bool:?}, rxne: {=bool:?}, tc: {=bool:?}, txe: {=bool:?}, lbd: {=bool:?}, ctsif: {=bool:?}, cts: {=bool:?}, rtof: {=bool:?}, eobf: {=bool:?}, abre: {=bool:?}, abrf: {=bool:?}, busy: {=bool:?}, cmf: {=bool:?}, sbkf: {=bool:?}, rwu: {:?}, wuf: {=bool:?}, teack: {=bool:?}, reack: {=bool:?} }}" , self . pe () , self . fe () , self . ne () , self . ore () , self . idle () , self . rxne () , self . tc () , self . txe () , self . lbd () , self . ctsif () , self . cts () , self . rtof () , self . eobf () , self . abre () , self . abrf () , self . busy () , self . cmf () , self . sbkf () , self . rwu () , self . wuf () , self . teack () , self . reack ())
        }
    }
    #[doc = "Request register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rqr(pub u32);
    impl Rqr {
        #[doc = "Auto baud rate request. Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame."]
        #[inline(always)]
        pub const fn abrrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Auto baud rate request. Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame."]
        #[inline(always)]
        pub fn set_abrrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Send break request. Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
        #[inline(always)]
        pub const fn sbkrq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Send break request. Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
        #[inline(always)]
        pub fn set_sbkrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mute mode request. Puts the USART in mute mode and sets the RWU flag."]
        #[inline(always)]
        pub const fn mmrq(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mute mode request. Puts the USART in mute mode and sets the RWU flag."]
        #[inline(always)]
        pub fn set_mmrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive data flush request. Clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
        #[inline(always)]
        pub const fn rxfrq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data flush request. Clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
        #[inline(always)]
        pub fn set_rxfrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit data flush request. Sets the TXE flags. This allows to discard the transmit data."]
        #[inline(always)]
        pub const fn txfrq(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data flush request. Sets the TXE flags. This allows to discard the transmit data."]
        #[inline(always)]
        pub fn set_txfrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Rqr {
        #[inline(always)]
        fn default() -> Rqr {
            Rqr(0)
        }
    }
    impl core::fmt::Debug for Rqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rqr")
                .field("abrrq", &self.abrrq())
                .field("sbkrq", &self.sbkrq())
                .field("mmrq", &self.mmrq())
                .field("rxfrq", &self.rxfrq())
                .field("txfrq", &self.txfrq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rqr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rqr {{ abrrq: {=bool:?}, sbkrq: {=bool:?}, mmrq: {=bool:?}, rxfrq: {=bool:?}, txfrq: {=bool:?} }}",
                self.abrrq(),
                self.sbkrq(),
                self.mmrq(),
                self.rxfrq(),
                self.txfrq()
            )
        }
    }
    #[doc = "Receiver timeout register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtor(pub u32);
    impl Rtor {
        #[doc = "Receiver timeout value"]
        #[inline(always)]
        pub const fn rto(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Receiver timeout value"]
        #[inline(always)]
        pub fn set_rto(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Block Length"]
        #[inline(always)]
        pub const fn blen(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Block Length"]
        #[inline(always)]
        pub fn set_blen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rtor {
        #[inline(always)]
        fn default() -> Rtor {
            Rtor(0)
        }
    }
    impl core::fmt::Debug for Rtor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtor")
                .field("rto", &self.rto())
                .field("blen", &self.blen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rtor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rtor {{ rto: {=u32:?}, blen: {=u8:?} }}", self.rto(), self.blen())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Abrmod {
        #[doc = "Measurement of the start bit is used to detect the baud rate"]
        START = 0x0,
        #[doc = "Falling edge to falling edge measurement"]
        EDGE = 0x01,
        #[doc = "0x7F frame detection"]
        FRAME7F = 0x02,
        #[doc = "0x55 frame detection"]
        FRAME55 = 0x03,
    }
    impl Abrmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Abrmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Abrmod {
        #[inline(always)]
        fn from(val: u8) -> Abrmod {
            Abrmod::from_bits(val)
        }
    }
    impl From<Abrmod> for u8 {
        #[inline(always)]
        fn from(val: Abrmod) -> u8 {
            Abrmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Addm {
        #[doc = "4-bit address detection"]
        BIT4 = 0x0,
        #[doc = "7-bit address detection"]
        BIT7 = 0x01,
    }
    impl Addm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Addm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Addm {
        #[inline(always)]
        fn from(val: u8) -> Addm {
            Addm::from_bits(val)
        }
    }
    impl From<Addm> for u8 {
        #[inline(always)]
        fn from(val: Addm) -> u8 {
            Addm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpha {
        #[doc = "The first clock transition is the first data capture edge"]
        FIRST = 0x0,
        #[doc = "The second clock transition is the first data capture edge"]
        SECOND = 0x01,
    }
    impl Cpha {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpha {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpha {
        #[inline(always)]
        fn from(val: u8) -> Cpha {
            Cpha::from_bits(val)
        }
    }
    impl From<Cpha> for u8 {
        #[inline(always)]
        fn from(val: Cpha) -> u8 {
            Cpha::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpol {
        #[doc = "Steady low value on CK pin outside transmission window"]
        LOW = 0x0,
        #[doc = "Steady high value on CK pin outside transmission window"]
        HIGH = 0x01,
    }
    impl Cpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpol {
        #[inline(always)]
        fn from(val: u8) -> Cpol {
            Cpol::from_bits(val)
        }
    }
    impl From<Cpol> for u8 {
        #[inline(always)]
        fn from(val: Cpol) -> u8 {
            Cpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dep {
        #[doc = "DE signal is active high"]
        HIGH = 0x0,
        #[doc = "DE signal is active low"]
        LOW = 0x01,
    }
    impl Dep {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dep {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dep {
        #[inline(always)]
        fn from(val: u8) -> Dep {
            Dep::from_bits(val)
        }
    }
    impl From<Dep> for u8 {
        #[inline(always)]
        fn from(val: Dep) -> u8 {
            Dep::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Irlp {
        #[doc = "Normal mode"]
        NORMAL = 0x0,
        #[doc = "Low-power mode"]
        LOW_POWER = 0x01,
    }
    impl Irlp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Irlp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Irlp {
        #[inline(always)]
        fn from(val: u8) -> Irlp {
            Irlp::from_bits(val)
        }
    }
    impl From<Irlp> for u8 {
        #[inline(always)]
        fn from(val: Irlp) -> u8 {
            Irlp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lbdl {
        #[doc = "10-bit break detection"]
        BIT10 = 0x0,
        #[doc = "11-bit break detection"]
        BIT11 = 0x01,
    }
    impl Lbdl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lbdl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lbdl {
        #[inline(always)]
        fn from(val: u8) -> Lbdl {
            Lbdl::from_bits(val)
        }
    }
    impl From<Lbdl> for u8 {
        #[inline(always)]
        fn from(val: Lbdl) -> u8 {
            Lbdl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum M0 {
        #[doc = "1 start bit, 8 data bits, n stop bits"]
        BIT8 = 0x0,
        #[doc = "1 start bit, 9 data bits, n stop bits"]
        BIT9 = 0x01,
    }
    impl M0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> M0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for M0 {
        #[inline(always)]
        fn from(val: u8) -> M0 {
            M0::from_bits(val)
        }
    }
    impl From<M0> for u8 {
        #[inline(always)]
        fn from(val: M0) -> u8 {
            M0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum M1 {
        #[doc = "Use M0 to set the data bits"]
        M0 = 0x0,
        #[doc = "1 start bit, 7 data bits, n stop bits"]
        BIT7 = 0x01,
    }
    impl M1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> M1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for M1 {
        #[inline(always)]
        fn from(val: u8) -> M1 {
            M1::from_bits(val)
        }
    }
    impl From<M1> for u8 {
        #[inline(always)]
        fn from(val: M1) -> u8 {
            M1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msbfirst {
        #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
        LSB = 0x0,
        #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
        MSB = 0x01,
    }
    impl Msbfirst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msbfirst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msbfirst {
        #[inline(always)]
        fn from(val: u8) -> Msbfirst {
            Msbfirst::from_bits(val)
        }
    }
    impl From<Msbfirst> for u8 {
        #[inline(always)]
        fn from(val: Msbfirst) -> u8 {
            Msbfirst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Over8 {
        #[doc = "Oversampling by 16"]
        OVERSAMPLING16 = 0x0,
        #[doc = "Oversampling by 8"]
        OVERSAMPLING8 = 0x01,
    }
    impl Over8 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Over8 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Over8 {
        #[inline(always)]
        fn from(val: u8) -> Over8 {
            Over8::from_bits(val)
        }
    }
    impl From<Over8> for u8 {
        #[inline(always)]
        fn from(val: Over8) -> u8 {
            Over8::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ps {
        #[doc = "Even parity"]
        EVEN = 0x0,
        #[doc = "Odd parity"]
        ODD = 0x01,
    }
    impl Ps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ps {
        #[inline(always)]
        fn from(val: u8) -> Ps {
            Ps::from_bits(val)
        }
    }
    impl From<Ps> for u8 {
        #[inline(always)]
        fn from(val: Ps) -> u8 {
            Ps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rwu {
        #[doc = "Receiver in active mode"]
        ACTIVE = 0x0,
        #[doc = "Receiver in mute mode"]
        MUTE = 0x01,
    }
    impl Rwu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rwu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rwu {
        #[inline(always)]
        fn from(val: u8) -> Rwu {
            Rwu::from_bits(val)
        }
    }
    impl From<Rwu> for u8 {
        #[inline(always)]
        fn from(val: Rwu) -> u8 {
            Rwu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stop {
        #[doc = "1 stop bit"]
        STOP1 = 0x0,
        #[doc = "0.5 stop bits"]
        STOP0P5 = 0x01,
        #[doc = "2 stop bits"]
        STOP2 = 0x02,
        #[doc = "1.5 stop bits"]
        STOP1P5 = 0x03,
    }
    impl Stop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stop {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stop {
        #[inline(always)]
        fn from(val: u8) -> Stop {
            Stop::from_bits(val)
        }
    }
    impl From<Stop> for u8 {
        #[inline(always)]
        fn from(val: Stop) -> u8 {
            Stop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wake {
        #[doc = "USART wakeup on idle line"]
        IDLE_LINE = 0x0,
        #[doc = "USART wakeup on address mark"]
        ADDRESS_MARK = 0x01,
    }
    impl Wake {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wake {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wake {
        #[inline(always)]
        fn from(val: u8) -> Wake {
            Wake::from_bits(val)
        }
    }
    impl From<Wake> for u8 {
        #[inline(always)]
        fn from(val: Wake) -> u8 {
            Wake::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wus {
        #[doc = "WUF active on address match"]
        ADDRESS = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "WuF active on Start bit detection"]
        START = 0x02,
        #[doc = "WUF active on RXNE"]
        RXNE = 0x03,
    }
    impl Wus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wus {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wus {
        #[inline(always)]
        fn from(val: u8) -> Wus {
            Wus::from_bits(val)
        }
    }
    impl From<Wus> for u8 {
        #[inline(always)]
        fn from(val: Wus) -> u8 {
            Wus::to_bits(val)
        }
    }
}
