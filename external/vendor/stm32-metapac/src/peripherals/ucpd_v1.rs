#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB Power Delivery interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucpd {
    ptr: *mut u8,
}
unsafe impl Send for Ucpd {}
unsafe impl Sync for Ucpd {}
impl Ucpd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "interrupt mask register"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Tx ordered set type register"]
    #[inline(always)]
    pub const fn tx_ordsetr(self) -> crate::common::Reg<regs::TxOrdsetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Tx payload size register"]
    #[inline(always)]
    pub const fn tx_payszr(self) -> crate::common::Reg<regs::TxPayszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Tx data register"]
    #[inline(always)]
    pub const fn txdr(self) -> crate::common::Reg<regs::Txdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ordsetr(self) -> crate::common::Reg<regs::RxOrdsetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_payszr(self) -> crate::common::Reg<regs::RxPayszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn rxdr(self) -> crate::common::Reg<regs::Rxdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Rx ordered set extension register 1"]
    #[inline(always)]
    pub const fn rx_ordextr1(self) -> crate::common::Reg<regs::RxOrdextr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Rx ordered set extension register 2"]
    #[inline(always)]
    pub const fn rx_ordextr2(self) -> crate::common::Reg<regs::RxOrdextr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "UCPD IP ID register"]
    #[inline(always)]
    pub const fn ipver(self) -> crate::common::Reg<regs::Ipver, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[doc = "UCPD IP ID register"]
    #[inline(always)]
    pub const fn ipid(self) -> crate::common::Reg<regs::Ipid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f8usize) as _) }
    }
    #[doc = "UCPD IP ID register"]
    #[inline(always)]
    pub const fn mid(self) -> crate::common::Reg<regs::Mid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a clk divider producing half-bit clock (hbit_clk)."]
        #[inline(always)]
        pub const fn hbitclkdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a clk divider producing half-bit clock (hbit_clk)."]
        #[inline(always)]
        pub fn set_hbitclkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal."]
        #[inline(always)]
        pub const fn ifrgap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal."]
        #[inline(always)]
        pub fn set_ifrgap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the clk frequency and the HBITCLKDIV\\[5:0\\]
bitfield setting."]
        #[inline(always)]
        pub const fn transwin(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the clk frequency and the HBITCLKDIV\\[5:0\\]
bitfield setting."]
        #[inline(always)]
        pub fn set_transwin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "Pre-scaler division ratio for generating clk The bitfield determines the division ratio of a kernel clock pre-scaler producing peripheral clock (clk). It is recommended to use the pre-scaler so as to set the clk frequency in the range from 6 to 9 MHz."]
        #[inline(always)]
        pub const fn psc_usbpdclk(&self) -> super::vals::PscUsbpdclk {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::PscUsbpdclk::from_bits(val as u8)
        }
        #[doc = "Pre-scaler division ratio for generating clk The bitfield determines the division ratio of a kernel clock pre-scaler producing peripheral clock (clk). It is recommended to use the pre-scaler so as to set the clk frequency in the range from 6 to 9 MHz."]
        #[inline(always)]
        pub fn set_psc_usbpdclk(&mut self, val: super::vals::PscUsbpdclk) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled"]
        #[inline(always)]
        pub const fn rxordseten(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x01ff;
            val as u16
        }
        #[doc = "Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled"]
        #[inline(always)]
        pub fn set_rxordseten(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
        }
        #[doc = "Transmission DMA mode enable When set, the bit enables DMA mode for transmission."]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission DMA mode enable When set, the bit enables DMA mode for transmission."]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reception DMA mode enable When set, the bit enables DMA mode for reception."]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reception DMA mode enable When set, the bit enables DMA mode for reception."]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "peripheral enable General enable of the peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state."]
        #[inline(always)]
        pub const fn ucpden(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "peripheral enable General enable of the peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state."]
        #[inline(always)]
        pub fn set_ucpden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("hbitclkdiv", &self.hbitclkdiv())
                .field("ifrgap", &self.ifrgap())
                .field("transwin", &self.transwin())
                .field("psc_usbpdclk", &self.psc_usbpdclk())
                .field("rxordseten", &self.rxordseten())
                .field("txdmaen", &self.txdmaen())
                .field("rxdmaen", &self.rxdmaen())
                .field("ucpden", &self.ucpden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr1 {{ hbitclkdiv: {=u8:?}, ifrgap: {=u8:?}, transwin: {=u8:?}, psc_usbpdclk: {:?}, rxordseten: {=u16:?}, txdmaen: {=bool:?}, rxdmaen: {=bool:?}, ucpden: {=bool:?} }}" , self . hbitclkdiv () , self . ifrgap () , self . transwin () , self . psc_usbpdclk () , self . rxordseten () , self . txdmaen () , self . rxdmaen () , self . ucpden ())
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler)."]
        #[inline(always)]
        pub const fn rxfiltdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler)."]
        #[inline(always)]
        pub fn set_rxfiltdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value."]
        #[inline(always)]
        pub const fn rxfilt2n3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value."]
        #[inline(always)]
        pub fn set_rxfilt2n3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force ClkReq clock request"]
        #[inline(always)]
        pub const fn forceclk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force ClkReq clock request"]
        #[inline(always)]
        pub fn set_forceclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup from Stop mode enable Setting the bit enables the ASYNC_INT signal."]
        #[inline(always)]
        pub const fn wupen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup from Stop mode enable Setting the bit enables the ASYNC_INT signal."]
        #[inline(always)]
        pub fn set_wupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("rxfiltdis", &self.rxfiltdis())
                .field("rxfilt2n3", &self.rxfilt2n3())
                .field("forceclk", &self.forceclk())
                .field("wupen", &self.wupen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ rxfiltdis: {=bool:?}, rxfilt2n3: {=bool:?}, forceclk: {=bool:?}, wupen: {=bool:?} }}",
                self.rxfiltdis(),
                self.rxfilt2n3(),
                self.forceclk(),
                self.wupen()
            )
        }
    }
    #[doc = "configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "SW trim value for Rd resistor on the CC1 line"]
        #[inline(always)]
        pub const fn trim_cc1_rd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SW trim value for Rd resistor on the CC1 line"]
        #[inline(always)]
        pub fn set_trim_cc1_rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SW trim value for Rp current sources on the CC1 line"]
        #[inline(always)]
        pub const fn trim_cc1_rp(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "SW trim value for Rp current sources on the CC1 line"]
        #[inline(always)]
        pub fn set_trim_cc1_rp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "SW trim value for Rd resistor on the CC2 line"]
        #[inline(always)]
        pub const fn trim_cc2_rd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SW trim value for Rd resistor on the CC2 line"]
        #[inline(always)]
        pub fn set_trim_cc2_rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "SW trim value for Rp current sources on the CC2 line"]
        #[inline(always)]
        pub const fn trim_cc2_rp(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "SW trim value for Rp current sources on the CC2 line"]
        #[inline(always)]
        pub fn set_trim_cc2_rp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
    }
    impl Default for Cfgr3 {
        #[inline(always)]
        fn default() -> Cfgr3 {
            Cfgr3(0)
        }
    }
    impl core::fmt::Debug for Cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr3")
                .field("trim_cc1_rd", &self.trim_cc1_rd())
                .field("trim_cc1_rp", &self.trim_cc1_rp())
                .field("trim_cc2_rd", &self.trim_cc2_rd())
                .field("trim_cc2_rp", &self.trim_cc2_rp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr3 {{ trim_cc1_rd: {=u8:?}, trim_cc1_rp: {=u8:?}, trim_cc2_rd: {=u8:?}, trim_cc2_rp: {=u8:?} }}",
                self.trim_cc1_rd(),
                self.trim_cc1_rp(),
                self.trim_cc2_rd(),
                self.trim_cc2_rp()
            )
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the \"tBISTContMode\" delay), disable the peripheral (UCPDEN = 0)."]
        #[inline(always)]
        pub const fn txmode(&self) -> super::vals::Txmode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Txmode::from_bits(val as u8)
        }
        #[doc = "Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the \"tBISTContMode\" delay), disable the peripheral (UCPDEN = 0)."]
        #[inline(always)]
        pub fn set_txmode(&mut self, val: super::vals::Txmode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded."]
        #[inline(always)]
        pub const fn txsend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded."]
        #[inline(always)]
        pub fn set_txsend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded."]
        #[inline(always)]
        pub const fn txhrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded."]
        #[inline(always)]
        pub fn set_txhrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message."]
        #[inline(always)]
        pub const fn rxmode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message."]
        #[inline(always)]
        pub fn set_rxmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set."]
        #[inline(always)]
        pub const fn phyrxen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set."]
        #[inline(always)]
        pub fn set_phyrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach."]
        #[inline(always)]
        pub const fn phyccsel(&self) -> super::vals::Phyccsel {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Phyccsel::from_bits(val as u8)
        }
        #[doc = "CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach."]
        #[inline(always)]
        pub fn set_phyccsel(&mut self, val: super::vals::Phyccsel) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield."]
        #[inline(always)]
        pub const fn anasubmode(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield."]
        #[inline(always)]
        pub fn set_anasubmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Analog PHY operating mode The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\\[1:0\\]."]
        #[inline(always)]
        pub const fn anamode(&self) -> super::vals::Anamode {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Anamode::from_bits(val as u8)
        }
        #[doc = "Analog PHY operating mode The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\\[1:0\\]."]
        #[inline(always)]
        pub fn set_anamode(&mut self, val: super::vals::Anamode) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\\[1:0\\]
setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source."]
        #[inline(always)]
        pub const fn ccenable(&self) -> super::vals::Ccenable {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Ccenable::from_bits(val as u8)
        }
        #[doc = "CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\\[1:0\\]
setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source."]
        #[inline(always)]
        pub fn set_ccenable(&mut self, val: super::vals::Ccenable) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "VCONN switch enable for CC1"]
        #[inline(always)]
        pub const fn cc1vconnen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "VCONN switch enable for CC1"]
        #[inline(always)]
        pub fn set_cc1vconnen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "VCONN switch enable for CC2"]
        #[inline(always)]
        pub const fn cc2vconnen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "VCONN switch enable for CC2"]
        #[inline(always)]
        pub fn set_cc2vconnen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured."]
        #[inline(always)]
        pub const fn dbatten(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured."]
        #[inline(always)]
        pub fn set_dbatten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink."]
        #[inline(always)]
        pub const fn frsrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink."]
        #[inline(always)]
        pub fn set_frsrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0."]
        #[inline(always)]
        pub const fn frstx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0."]
        #[inline(always)]
        pub fn set_frstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to \"USB Type-C ECN for Source VCONN Discharge\". The CCENABLE\\[1:0\\]
bitfield must be set accordingly, too."]
        #[inline(always)]
        pub const fn rdch(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to \"USB Type-C ECN for Source VCONN Discharge\". The CCENABLE\\[1:0\\]
bitfield must be set accordingly, too."]
        #[inline(always)]
        pub fn set_rdch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\\[1:0\\]."]
        #[inline(always)]
        pub const fn cc1tcdis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\\[1:0\\]."]
        #[inline(always)]
        pub fn set_cc1tcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\\[1:0\\]."]
        #[inline(always)]
        pub const fn cc2tcdis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\\[1:0\\]."]
        #[inline(always)]
        pub fn set_cc2tcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
            f.debug_struct("Cr")
                .field("txmode", &self.txmode())
                .field("txsend", &self.txsend())
                .field("txhrst", &self.txhrst())
                .field("rxmode", &self.rxmode())
                .field("phyrxen", &self.phyrxen())
                .field("phyccsel", &self.phyccsel())
                .field("anasubmode", &self.anasubmode())
                .field("anamode", &self.anamode())
                .field("ccenable", &self.ccenable())
                .field("cc1vconnen", &self.cc1vconnen())
                .field("cc2vconnen", &self.cc2vconnen())
                .field("dbatten", &self.dbatten())
                .field("frsrxen", &self.frsrxen())
                .field("frstx", &self.frstx())
                .field("rdch", &self.rdch())
                .field("cc1tcdis", &self.cc1tcdis())
                .field("cc2tcdis", &self.cc2tcdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ txmode: {:?}, txsend: {=bool:?}, txhrst: {=bool:?}, rxmode: {=bool:?}, phyrxen: {=bool:?}, phyccsel: {:?}, anasubmode: {=u8:?}, anamode: {:?}, ccenable: {:?}, cc1vconnen: {=bool:?}, cc2vconnen: {=bool:?}, dbatten: {=bool:?}, frsrxen: {=bool:?}, frstx: {=bool:?}, rdch: {=bool:?}, cc1tcdis: {=bool:?}, cc2tcdis: {=bool:?} }}" , self . txmode () , self . txsend () , self . txhrst () , self . rxmode () , self . phyrxen () , self . phyccsel () , self . anasubmode () , self . anamode () , self . ccenable () , self . cc1vconnen () , self . cc2vconnen () , self . dbatten () , self . frsrxen () , self . frstx () , self . rdch () , self . cc1tcdis () , self . cc2tcdis ())
        }
    }
    #[doc = "interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the SR register."]
        #[inline(always)]
        pub const fn txmsgdisccf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tx message discard flag (TXMSGDISC) clear Setting the bit clears the TXMSGDISC flag in the SR register."]
        #[inline(always)]
        pub fn set_txmsgdisccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the SR register."]
        #[inline(always)]
        pub const fn txmsgsentcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Tx message send flag (TXMSGSENT) clear Setting the bit clears the TXMSGSENT flag in the SR register."]
        #[inline(always)]
        pub fn set_txmsgsentcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the SR register."]
        #[inline(always)]
        pub const fn txmsgabtcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Tx message abort flag (TXMSGABT) clear Setting the bit clears the TXMSGABT flag in the SR register."]
        #[inline(always)]
        pub fn set_txmsgabtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the SR register."]
        #[inline(always)]
        pub const fn hrstdisccf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Hard reset discard flag (HRSTDISC) clear Setting the bit clears the HRSTDISC flag in the SR register."]
        #[inline(always)]
        pub fn set_hrstdisccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the SR register."]
        #[inline(always)]
        pub const fn hrstsentcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hard reset send flag (HRSTSENT) clear Setting the bit clears the HRSTSENT flag in the SR register."]
        #[inline(always)]
        pub fn set_hrstsentcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the SR register."]
        #[inline(always)]
        pub const fn txundcf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Tx underflow flag (TXUND) clear Setting the bit clears the TXUND flag in the SR register."]
        #[inline(always)]
        pub fn set_txundcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the SR register."]
        #[inline(always)]
        pub const fn rxorddetcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Rx ordered set detect flag (RXORDDET) clear Setting the bit clears the RXORDDET flag in the SR register."]
        #[inline(always)]
        pub fn set_rxorddetcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the SR register."]
        #[inline(always)]
        pub const fn rxhrstdetcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Rx Hard Reset detect flag (RXHRSTDET) clear Setting the bit clears the RXHRSTDET flag in the SR register."]
        #[inline(always)]
        pub fn set_rxhrstdetcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the SR register."]
        #[inline(always)]
        pub const fn rxovrcf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Rx overflow flag (RXOVR) clear Setting the bit clears the RXOVR flag in the SR register."]
        #[inline(always)]
        pub fn set_rxovrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the SR register."]
        #[inline(always)]
        pub const fn rxmsgendcf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Rx message received flag (RXMSGEND) clear Setting the bit clears the RXMSGEND flag in the SR register."]
        #[inline(always)]
        pub fn set_rxmsgendcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the SR register"]
        #[inline(always)]
        pub const fn typecevt1cf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Type-C CC1 event flag (TYPECEVT1) clear Setting the bit clears the TYPECEVT1 flag in the SR register"]
        #[inline(always)]
        pub fn set_typecevt1cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the SR register"]
        #[inline(always)]
        pub const fn typecevt2cf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Type-C CC2 line event flag (TYPECEVT2) clear Setting the bit clears the TYPECEVT2 flag in the SR register"]
        #[inline(always)]
        pub fn set_typecevt2cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the SR register."]
        #[inline(always)]
        pub const fn frsevtcf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "FRS event flag (FRSEVT) clear Setting the bit clears the FRSEVT flag in the SR register."]
        #[inline(always)]
        pub fn set_frsevtcf(&mut self, val: bool) {
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
                .field("txmsgdisccf", &self.txmsgdisccf())
                .field("txmsgsentcf", &self.txmsgsentcf())
                .field("txmsgabtcf", &self.txmsgabtcf())
                .field("hrstdisccf", &self.hrstdisccf())
                .field("hrstsentcf", &self.hrstsentcf())
                .field("txundcf", &self.txundcf())
                .field("rxorddetcf", &self.rxorddetcf())
                .field("rxhrstdetcf", &self.rxhrstdetcf())
                .field("rxovrcf", &self.rxovrcf())
                .field("rxmsgendcf", &self.rxmsgendcf())
                .field("typecevt1cf", &self.typecevt1cf())
                .field("typecevt2cf", &self.typecevt2cf())
                .field("frsevtcf", &self.frsevtcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ txmsgdisccf: {=bool:?}, txmsgsentcf: {=bool:?}, txmsgabtcf: {=bool:?}, hrstdisccf: {=bool:?}, hrstsentcf: {=bool:?}, txundcf: {=bool:?}, rxorddetcf: {=bool:?}, rxhrstdetcf: {=bool:?}, rxovrcf: {=bool:?}, rxmsgendcf: {=bool:?}, typecevt1cf: {=bool:?}, typecevt2cf: {=bool:?}, frsevtcf: {=bool:?} }}" , self . txmsgdisccf () , self . txmsgsentcf () , self . txmsgabtcf () , self . hrstdisccf () , self . hrstsentcf () , self . txundcf () , self . rxorddetcf () , self . rxhrstdetcf () , self . rxovrcf () , self . rxmsgendcf () , self . typecevt1cf () , self . typecevt2cf () , self . frsevtcf ())
        }
    }
    #[doc = "interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr(pub u32);
    impl Imr {
        #[doc = "TXIS interrupt enable"]
        #[inline(always)]
        pub const fn txisie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TXIS interrupt enable"]
        #[inline(always)]
        pub fn set_txisie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TXMSGDISC interrupt enable"]
        #[inline(always)]
        pub const fn txmsgdiscie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TXMSGDISC interrupt enable"]
        #[inline(always)]
        pub fn set_txmsgdiscie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TXMSGSENT interrupt enable"]
        #[inline(always)]
        pub const fn txmsgsentie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TXMSGSENT interrupt enable"]
        #[inline(always)]
        pub fn set_txmsgsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TXMSGABT interrupt enable"]
        #[inline(always)]
        pub const fn txmsgabtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TXMSGABT interrupt enable"]
        #[inline(always)]
        pub fn set_txmsgabtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HRSTDISC interrupt enable"]
        #[inline(always)]
        pub const fn hrstdiscie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HRSTDISC interrupt enable"]
        #[inline(always)]
        pub fn set_hrstdiscie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HRSTSENT interrupt enable"]
        #[inline(always)]
        pub const fn hrstsentie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HRSTSENT interrupt enable"]
        #[inline(always)]
        pub fn set_hrstsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TXUND interrupt enable"]
        #[inline(always)]
        pub const fn txundie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TXUND interrupt enable"]
        #[inline(always)]
        pub fn set_txundie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RXORDDET interrupt enable"]
        #[inline(always)]
        pub const fn rxorddetie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RXORDDET interrupt enable"]
        #[inline(always)]
        pub fn set_rxorddetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RXHRSTDET interrupt enable"]
        #[inline(always)]
        pub const fn rxhrstdetie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RXHRSTDET interrupt enable"]
        #[inline(always)]
        pub fn set_rxhrstdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "RXOVR interrupt enable"]
        #[inline(always)]
        pub const fn rxovrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RXOVR interrupt enable"]
        #[inline(always)]
        pub fn set_rxovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RXMSGEND interrupt enable"]
        #[inline(always)]
        pub const fn rxmsgendie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RXMSGEND interrupt enable"]
        #[inline(always)]
        pub fn set_rxmsgendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TYPECEVT1 interrupt enable"]
        #[inline(always)]
        pub const fn typecevt1ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TYPECEVT1 interrupt enable"]
        #[inline(always)]
        pub fn set_typecevt1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TYPECEVT2 interrupt enable"]
        #[inline(always)]
        pub const fn typecevt2ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TYPECEVT2 interrupt enable"]
        #[inline(always)]
        pub fn set_typecevt2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "FRSEVT interrupt enable"]
        #[inline(always)]
        pub const fn frsevtie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "FRSEVT interrupt enable"]
        #[inline(always)]
        pub fn set_frsevtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Imr {
        #[inline(always)]
        fn default() -> Imr {
            Imr(0)
        }
    }
    impl core::fmt::Debug for Imr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr")
                .field("txisie", &self.txisie())
                .field("txmsgdiscie", &self.txmsgdiscie())
                .field("txmsgsentie", &self.txmsgsentie())
                .field("txmsgabtie", &self.txmsgabtie())
                .field("hrstdiscie", &self.hrstdiscie())
                .field("hrstsentie", &self.hrstsentie())
                .field("txundie", &self.txundie())
                .field("rxneie", &self.rxneie())
                .field("rxorddetie", &self.rxorddetie())
                .field("rxhrstdetie", &self.rxhrstdetie())
                .field("rxovrie", &self.rxovrie())
                .field("rxmsgendie", &self.rxmsgendie())
                .field("typecevt1ie", &self.typecevt1ie())
                .field("typecevt2ie", &self.typecevt2ie())
                .field("frsevtie", &self.frsevtie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Imr {{ txisie: {=bool:?}, txmsgdiscie: {=bool:?}, txmsgsentie: {=bool:?}, txmsgabtie: {=bool:?}, hrstdiscie: {=bool:?}, hrstsentie: {=bool:?}, txundie: {=bool:?}, rxneie: {=bool:?}, rxorddetie: {=bool:?}, rxhrstdetie: {=bool:?}, rxovrie: {=bool:?}, rxmsgendie: {=bool:?}, typecevt1ie: {=bool:?}, typecevt2ie: {=bool:?}, frsevtie: {=bool:?} }}" , self . txisie () , self . txmsgdiscie () , self . txmsgsentie () , self . txmsgabtie () , self . hrstdiscie () , self . hrstsentie () , self . txundie () , self . rxneie () , self . rxorddetie () , self . rxhrstdetie () , self . rxovrie () , self . rxmsgendie () , self . typecevt1ie () , self . typecevt2ie () , self . frsevtie ())
        }
    }
    #[doc = "UCPD IP ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipid(pub u32);
    impl Ipid {
        #[doc = "IPID"]
        #[inline(always)]
        pub const fn ipid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IPID"]
        #[inline(always)]
        pub fn set_ipid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipid {
        #[inline(always)]
        fn default() -> Ipid {
            Ipid(0)
        }
    }
    impl core::fmt::Debug for Ipid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipid").field("ipid", &self.ipid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipid {{ ipid: {=u32:?} }}", self.ipid())
        }
    }
    #[doc = "UCPD IP ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipver(pub u32);
    impl Ipver {
        #[doc = "IPVER"]
        #[inline(always)]
        pub const fn ipver(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IPVER"]
        #[inline(always)]
        pub fn set_ipver(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipver {
        #[inline(always)]
        fn default() -> Ipver {
            Ipver(0)
        }
    }
    impl core::fmt::Debug for Ipver {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipver").field("ipver", &self.ipver()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipver {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipver {{ ipver: {=u32:?} }}", self.ipver())
        }
    }
    #[doc = "UCPD IP ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mid(pub u32);
    impl Mid {
        #[doc = "IPID"]
        #[inline(always)]
        pub const fn ipid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IPID"]
        #[inline(always)]
        pub fn set_ipid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mid {
        #[inline(always)]
        fn default() -> Mid {
            Mid(0)
        }
    }
    impl core::fmt::Debug for Mid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mid").field("ipid", &self.ipid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mid {{ ipid: {=u32:?} }}", self.ipid())
        }
    }
    #[doc = "Rx ordered set extension register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxOrdextr1(pub u32);
    impl RxOrdextr1 {
        #[doc = "Ordered set 1 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
        #[inline(always)]
        pub const fn rxsopx1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Ordered set 1 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
        #[inline(always)]
        pub fn set_rxsopx1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for RxOrdextr1 {
        #[inline(always)]
        fn default() -> RxOrdextr1 {
            RxOrdextr1(0)
        }
    }
    impl core::fmt::Debug for RxOrdextr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxOrdextr1").field("rxsopx1", &self.rxsopx1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxOrdextr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxOrdextr1 {{ rxsopx1: {=u32:?} }}", self.rxsopx1())
        }
    }
    #[doc = "Rx ordered set extension register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxOrdextr2(pub u32);
    impl RxOrdextr2 {
        #[doc = "Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
        #[inline(always)]
        pub const fn rxsopx2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
        #[inline(always)]
        pub fn set_rxsopx2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for RxOrdextr2 {
        #[inline(always)]
        fn default() -> RxOrdextr2 {
            RxOrdextr2(0)
        }
    }
    impl core::fmt::Debug for RxOrdextr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxOrdextr2").field("rxsopx2", &self.rxsopx2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxOrdextr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxOrdextr2 {{ rxsopx2: {=u32:?} }}", self.rxsopx2())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxOrdsetr(pub u32);
    impl RxOrdsetr {
        #[doc = "Rx ordered set code detected"]
        #[inline(always)]
        pub const fn rxordset(&self) -> super::vals::Rxordset {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Rxordset::from_bits(val as u8)
        }
        #[doc = "Rx ordered set code detected"]
        #[inline(always)]
        pub fn set_rxordset(&mut self, val: super::vals::Rxordset) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "The bit indicates the number of correct K‑codes. For debug purposes only."]
        #[inline(always)]
        pub const fn rxsop3of4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The bit indicates the number of correct K‑codes. For debug purposes only."]
        #[inline(always)]
        pub fn set_rxsop3of4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "The bitfield is for debug purposes only. Others: Invalid"]
        #[inline(always)]
        pub const fn rxsopkinvalid(&self) -> super::vals::Rxsopkinvalid {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Rxsopkinvalid::from_bits(val as u8)
        }
        #[doc = "The bitfield is for debug purposes only. Others: Invalid"]
        #[inline(always)]
        pub fn set_rxsopkinvalid(&mut self, val: super::vals::Rxsopkinvalid) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
    }
    impl Default for RxOrdsetr {
        #[inline(always)]
        fn default() -> RxOrdsetr {
            RxOrdsetr(0)
        }
    }
    impl core::fmt::Debug for RxOrdsetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxOrdsetr")
                .field("rxordset", &self.rxordset())
                .field("rxsop3of4", &self.rxsop3of4())
                .field("rxsopkinvalid", &self.rxsopkinvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxOrdsetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RxOrdsetr {{ rxordset: {:?}, rxsop3of4: {=bool:?}, rxsopkinvalid: {:?} }}",
                self.rxordset(),
                self.rxsop3of4(),
                self.rxsopkinvalid()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxPayszr(pub u32);
    impl RxPayszr {
        #[doc = "Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low)."]
        #[inline(always)]
        pub const fn rxpaysz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low)."]
        #[inline(always)]
        pub fn set_rxpaysz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for RxPayszr {
        #[inline(always)]
        fn default() -> RxPayszr {
            RxPayszr(0)
        }
    }
    impl core::fmt::Debug for RxPayszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxPayszr").field("rxpaysz", &self.rxpaysz()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxPayszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxPayszr {{ rxpaysz: {=u16:?} }}", self.rxpaysz())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdr(pub u32);
    impl Rxdr {
        #[doc = "Data byte received"]
        #[inline(always)]
        pub const fn rxdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte received"]
        #[inline(always)]
        pub fn set_rxdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rxdr {
        #[inline(always)]
        fn default() -> Rxdr {
            Rxdr(0)
        }
    }
    impl core::fmt::Debug for Rxdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdr").field("rxdata", &self.rxdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxdr {{ rxdata: {=u8:?} }}", self.rxdata())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Transmit interrupt status The flag indicates that the TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the TXDR register."]
        #[inline(always)]
        pub const fn txis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit interrupt status The flag indicates that the TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the TXDR register."]
        #[inline(always)]
        pub fn set_txis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle."]
        #[inline(always)]
        pub const fn txmsgdisc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle."]
        #[inline(always)]
        pub fn set_txmsgdisc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised."]
        #[inline(always)]
        pub const fn txmsgsent(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised."]
        #[inline(always)]
        pub fn set_txmsgsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit."]
        #[inline(always)]
        pub const fn txmsgabt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit."]
        #[inline(always)]
        pub fn set_txmsgabt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit."]
        #[inline(always)]
        pub const fn hrstdisc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit."]
        #[inline(always)]
        pub fn set_hrstdisc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit."]
        #[inline(always)]
        pub const fn hrstsent(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit."]
        #[inline(always)]
        pub fn set_hrstsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Tx data underrun detection The flag indicates that the Tx data register (TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit."]
        #[inline(always)]
        pub const fn txund(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Tx data underrun detection The flag indicates that the Tx data register (TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit."]
        #[inline(always)]
        pub fn set_txund(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive data register not empty detection The flag indicates that the RXDR register is not empty. It is automatically cleared upon reading RXDR."]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive data register not empty detection The flag indicates that the RXDR register is not empty. It is automatically cleared upon reading RXDR."]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\\[2:0\\]
bitfield of the RX_ORDSET register. It is cleared by setting the RXORDDETCF bit."]
        #[inline(always)]
        pub const fn rxorddet(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\\[2:0\\]
bitfield of the RX_ORDSET register. It is cleared by setting the RXORDDETCF bit."]
        #[inline(always)]
        pub fn set_rxorddet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit."]
        #[inline(always)]
        pub const fn rxhrstdet(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit."]
        #[inline(always)]
        pub fn set_rxhrstdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough."]
        #[inline(always)]
        pub const fn rxovr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough."]
        #[inline(always)]
        pub fn set_rxovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message."]
        #[inline(always)]
        pub const fn rxmsgend(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message."]
        #[inline(always)]
        pub fn set_rxmsgend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set."]
        #[inline(always)]
        pub const fn rxerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set."]
        #[inline(always)]
        pub fn set_rxerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
        #[inline(always)]
        pub const fn typecevt1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
        #[inline(always)]
        pub fn set_typecevt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
        #[inline(always)]
        pub const fn typecevt2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
        #[inline(always)]
        pub fn set_typecevt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
        #[inline(always)]
        pub const fn typec_vstate_cc1(&self) -> super::vals::TypecVstateCc {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::TypecVstateCc::from_bits(val as u8)
        }
        #[doc = "The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
        #[inline(always)]
        pub fn set_typec_vstate_cc1(&mut self, val: super::vals::TypecVstateCc) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
        #[inline(always)]
        pub const fn typec_vstate_cc2(&self) -> super::vals::TypecVstateCc {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::TypecVstateCc::from_bits(val as u8)
        }
        #[doc = "CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
        #[inline(always)]
        pub fn set_typec_vstate_cc2(&mut self, val: super::vals::TypecVstateCc) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "FRS detection event The flag is cleared by setting the FRSEVTCF bit."]
        #[inline(always)]
        pub const fn frsevt(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "FRS detection event The flag is cleared by setting the FRSEVTCF bit."]
        #[inline(always)]
        pub fn set_frsevt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("txis", &self.txis())
                .field("txmsgdisc", &self.txmsgdisc())
                .field("txmsgsent", &self.txmsgsent())
                .field("txmsgabt", &self.txmsgabt())
                .field("hrstdisc", &self.hrstdisc())
                .field("hrstsent", &self.hrstsent())
                .field("txund", &self.txund())
                .field("rxne", &self.rxne())
                .field("rxorddet", &self.rxorddet())
                .field("rxhrstdet", &self.rxhrstdet())
                .field("rxovr", &self.rxovr())
                .field("rxmsgend", &self.rxmsgend())
                .field("rxerr", &self.rxerr())
                .field("typecevt1", &self.typecevt1())
                .field("typecevt2", &self.typecevt2())
                .field("typec_vstate_cc1", &self.typec_vstate_cc1())
                .field("typec_vstate_cc2", &self.typec_vstate_cc2())
                .field("frsevt", &self.frsevt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ txis: {=bool:?}, txmsgdisc: {=bool:?}, txmsgsent: {=bool:?}, txmsgabt: {=bool:?}, hrstdisc: {=bool:?}, hrstsent: {=bool:?}, txund: {=bool:?}, rxne: {=bool:?}, rxorddet: {=bool:?}, rxhrstdet: {=bool:?}, rxovr: {=bool:?}, rxmsgend: {=bool:?}, rxerr: {=bool:?}, typecevt1: {=bool:?}, typecevt2: {=bool:?}, typec_vstate_cc1: {:?}, typec_vstate_cc2: {:?}, frsevt: {=bool:?} }}" , self . txis () , self . txmsgdisc () , self . txmsgsent () , self . txmsgabt () , self . hrstdisc () , self . hrstsent () , self . txund () , self . rxne () , self . rxorddet () , self . rxhrstdet () , self . rxovr () , self . rxmsgend () , self . rxerr () , self . typecevt1 () , self . typecevt2 () , self . typec_vstate_cc1 () , self . typec_vstate_cc2 () , self . frsevt ())
        }
    }
    #[doc = "Tx ordered set type register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxOrdsetr(pub u32);
    impl TxOrdsetr {
        #[doc = "Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
        #[inline(always)]
        pub const fn txordset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
        #[inline(always)]
        pub fn set_txordset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for TxOrdsetr {
        #[inline(always)]
        fn default() -> TxOrdsetr {
            TxOrdsetr(0)
        }
    }
    impl core::fmt::Debug for TxOrdsetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxOrdsetr").field("txordset", &self.txordset()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxOrdsetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxOrdsetr {{ txordset: {=u32:?} }}", self.txordset())
        }
    }
    #[doc = "Tx payload size register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxPayszr(pub u32);
    impl TxPayszr {
        #[doc = "Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
        #[inline(always)]
        pub const fn txpaysz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
        #[inline(always)]
        pub fn set_txpaysz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for TxPayszr {
        #[inline(always)]
        fn default() -> TxPayszr {
            TxPayszr(0)
        }
    }
    impl core::fmt::Debug for TxPayszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxPayszr").field("txpaysz", &self.txpaysz()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxPayszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxPayszr {{ txpaysz: {=u16:?} }}", self.txpaysz())
        }
    }
    #[doc = "Tx data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdr(pub u32);
    impl Txdr {
        #[doc = "Data byte to transmit"]
        #[inline(always)]
        pub const fn txdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte to transmit"]
        #[inline(always)]
        pub fn set_txdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Txdr {
        #[inline(always)]
        fn default() -> Txdr {
            Txdr(0)
        }
    }
    impl core::fmt::Debug for Txdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdr").field("txdata", &self.txdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txdr {{ txdata: {=u8:?} }}", self.txdata())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Anamode {
        #[doc = "Source"]
        SOURCE = 0x0,
        #[doc = "Sink"]
        SINK = 0x01,
    }
    impl Anamode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Anamode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Anamode {
        #[inline(always)]
        fn from(val: u8) -> Anamode {
            Anamode::from_bits(val)
        }
    }
    impl From<Anamode> for u8 {
        #[inline(always)]
        fn from(val: Anamode) -> u8 {
            Anamode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ccenable {
        #[doc = "Disable both PHYs"]
        DISABLED = 0x0,
        #[doc = "Enable CC1 PHY"]
        CC1 = 0x01,
        #[doc = "Enable CC2 PHY"]
        CC2 = 0x02,
        #[doc = "Enable CC1 and CC2 PHY"]
        BOTH = 0x03,
    }
    impl Ccenable {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccenable {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccenable {
        #[inline(always)]
        fn from(val: u8) -> Ccenable {
            Ccenable::from_bits(val)
        }
    }
    impl From<Ccenable> for u8 {
        #[inline(always)]
        fn from(val: Ccenable) -> u8 {
            Ccenable::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Phyccsel {
        #[doc = "Use CC1 IO for Power Delivery communication"]
        CC1 = 0x0,
        #[doc = "Use CC2 IO for Power Delivery communication"]
        CC2 = 0x01,
    }
    impl Phyccsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Phyccsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Phyccsel {
        #[inline(always)]
        fn from(val: u8) -> Phyccsel {
            Phyccsel::from_bits(val)
        }
    }
    impl From<Phyccsel> for u8 {
        #[inline(always)]
        fn from(val: Phyccsel) -> u8 {
            Phyccsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PscUsbpdclk {
        #[doc = "1 (bypass)"]
        DIV1 = 0x0,
        #[doc = "2"]
        DIV2 = 0x01,
        #[doc = "4"]
        DIV4 = 0x02,
        #[doc = "8"]
        DIV8 = 0x03,
        #[doc = "16"]
        DIV16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl PscUsbpdclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PscUsbpdclk {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PscUsbpdclk {
        #[inline(always)]
        fn from(val: u8) -> PscUsbpdclk {
            PscUsbpdclk::from_bits(val)
        }
    }
    impl From<PscUsbpdclk> for u8 {
        #[inline(always)]
        fn from(val: PscUsbpdclk) -> u8 {
            PscUsbpdclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxordset {
        #[doc = "SOP code detected in receiver"]
        SOP = 0x0,
        #[doc = "SOP' code detected in receiver"]
        SOP_PRIME = 0x01,
        #[doc = "SOP'' code detected in receiver"]
        SOP_DOUBLE_PRIME = 0x02,
        #[doc = "SOP'_Debug detected in receiver"]
        SOP_PRIME_DEBUG = 0x03,
        #[doc = "SOP''_Debug detected in receiver"]
        SOP_DOUBLE_PRIME_DEBUG = 0x04,
        #[doc = "Cable Reset detected in receiver"]
        CABLE_RESET = 0x05,
        #[doc = "SOP extension#1 detected in receiver"]
        EXT1 = 0x06,
        #[doc = "SOP extension#2 detected in receiver"]
        EXT2 = 0x07,
    }
    impl Rxordset {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxordset {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxordset {
        #[inline(always)]
        fn from(val: u8) -> Rxordset {
            Rxordset::from_bits(val)
        }
    }
    impl From<Rxordset> for u8 {
        #[inline(always)]
        fn from(val: Rxordset) -> u8 {
            Rxordset::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxsopkinvalid {
        #[doc = "No K‑code corrupted"]
        NONE = 0x0,
        #[doc = "First K‑code corrupted"]
        FIRST = 0x01,
        #[doc = "Second K‑code corrupted"]
        SECOND = 0x02,
        #[doc = "Third K‑code corrupted"]
        THIRD = 0x03,
        #[doc = "Fourth K‑code corrupted"]
        FOURTH = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Rxsopkinvalid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxsopkinvalid {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxsopkinvalid {
        #[inline(always)]
        fn from(val: u8) -> Rxsopkinvalid {
            Rxsopkinvalid::from_bits(val)
        }
    }
    impl From<Rxsopkinvalid> for u8 {
        #[inline(always)]
        fn from(val: Rxsopkinvalid) -> u8 {
            Rxsopkinvalid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Txmode {
        #[doc = "Transmission of Tx packet previously defined in other registers"]
        PACKET = 0x0,
        #[doc = "Cable Reset sequence"]
        CABLE_RESET = 0x01,
        #[doc = "BIST test sequence (BIST Carrier Mode 2)"]
        BIST = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Txmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Txmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Txmode {
        #[inline(always)]
        fn from(val: u8) -> Txmode {
            Txmode::from_bits(val)
        }
    }
    impl From<Txmode> for u8 {
        #[inline(always)]
        fn from(val: Txmode) -> u8 {
            Txmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TypecVstateCc {
        #[doc = "Lowest"]
        LOWEST = 0x0,
        #[doc = "Low"]
        LOW = 0x01,
        #[doc = "High"]
        HIGH = 0x02,
        #[doc = "Highest"]
        HIGHEST = 0x03,
    }
    impl TypecVstateCc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TypecVstateCc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TypecVstateCc {
        #[inline(always)]
        fn from(val: u8) -> TypecVstateCc {
            TypecVstateCc::from_bits(val)
        }
    }
    impl From<TypecVstateCc> for u8 {
        #[inline(always)]
        fn from(val: TypecVstateCc) -> u8 {
            TypecVstateCc::to_bits(val)
        }
    }
}
