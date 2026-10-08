#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DIE_ID register."]
    #[inline(always)]
    pub const fn die_id(self) -> crate::common::Reg<regs::DieId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "JTAG_ID register."]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::common::Reg<regs::JtagId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "I2C_FMP_CTRL register."]
    #[inline(always)]
    pub const fn i2c_fmp_ctrl(self) -> crate::common::Reg<regs::I2cFmpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "IO_DTR register."]
    #[inline(always)]
    pub const fn io_dtr(self) -> crate::common::Reg<regs::IoDtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "IO_IBER register."]
    #[inline(always)]
    pub const fn io_iber(self) -> crate::common::Reg<regs::IoIber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "IO_IEVR register."]
    #[inline(always)]
    pub const fn io_ievr(self) -> crate::common::Reg<regs::IoIevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "IO_IER register."]
    #[inline(always)]
    pub const fn io_ier(self) -> crate::common::Reg<regs::IoIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "IO_ISCR register."]
    #[inline(always)]
    pub const fn io_iscr(self) -> crate::common::Reg<regs::IoIscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PWRC_IER register."]
    #[inline(always)]
    pub const fn pwrc_ier(self) -> crate::common::Reg<regs::PwrcIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PWRC_ISCR register."]
    #[inline(always)]
    pub const fn pwrc_iscr(self) -> crate::common::Reg<regs::PwrcIscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "BLERXTX_DTR register."]
    #[inline(always)]
    pub const fn blerxtx_dtr(self) -> crate::common::Reg<regs::BlerxtxDtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "BLERXTX_IBER register."]
    #[inline(always)]
    pub const fn blerxtx_iber(self) -> crate::common::Reg<regs::BlerxtxIber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "BLERXTX_IEVR register."]
    #[inline(always)]
    pub const fn blerxtx_ievr(self) -> crate::common::Reg<regs::BlerxtxIevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "BLERXTX_IER register."]
    #[inline(always)]
    pub const fn blerxtx_ier(self) -> crate::common::Reg<regs::BlerxtxIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "BLERXTX_ISCR register."]
    #[inline(always)]
    pub const fn blerxtx_iscr(self) -> crate::common::Reg<regs::BlerxtxIscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "BLERXTX_DTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlerxtxDtr(pub u32);
    impl BlerxtxDtr {
        #[doc = "TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level."]
        #[inline(always)]
        pub const fn tx_dt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level."]
        #[inline(always)]
        pub fn set_tx_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level."]
        #[inline(always)]
        pub const fn rx_dt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level."]
        #[inline(always)]
        pub fn set_rx_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BlerxtxDtr {
        #[inline(always)]
        fn default() -> BlerxtxDtr {
            BlerxtxDtr(0)
        }
    }
    impl core::fmt::Debug for BlerxtxDtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlerxtxDtr")
                .field("tx_dt", &self.tx_dt())
                .field("rx_dt", &self.rx_dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlerxtxDtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BlerxtxDtr {{ tx_dt: {=bool:?}, rx_dt: {=bool:?} }}",
                self.tx_dt(),
                self.rx_dt()
            )
        }
    }
    #[doc = "BLERXTX_IBER register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlerxtxIber(pub u32);
    impl BlerxtxIber {
        #[doc = "TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges."]
        #[inline(always)]
        pub const fn tx_ibe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges."]
        #[inline(always)]
        pub fn set_tx_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges."]
        #[inline(always)]
        pub const fn rx_ibe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges."]
        #[inline(always)]
        pub fn set_rx_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BlerxtxIber {
        #[inline(always)]
        fn default() -> BlerxtxIber {
            BlerxtxIber(0)
        }
    }
    impl core::fmt::Debug for BlerxtxIber {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlerxtxIber")
                .field("tx_ibe", &self.tx_ibe())
                .field("rx_ibe", &self.rx_ibe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlerxtxIber {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BlerxtxIber {{ tx_ibe: {=bool:?}, rx_ibe: {=bool:?} }}",
                self.tx_ibe(),
                self.rx_ibe()
            )
        }
    }
    #[doc = "BLERXTX_IER register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlerxtxIer(pub u32);
    impl BlerxtxIer {
        #[doc = "TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled."]
        #[inline(always)]
        pub const fn tx_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled."]
        #[inline(always)]
        pub fn set_tx_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled."]
        #[inline(always)]
        pub const fn rx_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled."]
        #[inline(always)]
        pub fn set_rx_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BlerxtxIer {
        #[inline(always)]
        fn default() -> BlerxtxIer {
            BlerxtxIer(0)
        }
    }
    impl core::fmt::Debug for BlerxtxIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlerxtxIer")
                .field("tx_ie", &self.tx_ie())
                .field("rx_ie", &self.rx_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlerxtxIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BlerxtxIer {{ tx_ie: {=bool:?}, rx_ie: {=bool:?} }}",
                self.tx_ie(),
                self.rx_ie()
            )
        }
    }
    #[doc = "BLERXTX_IEVR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlerxtxIevr(pub u32);
    impl BlerxtxIevr {
        #[doc = "TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level."]
        #[inline(always)]
        pub const fn tx_iev(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level."]
        #[inline(always)]
        pub fn set_tx_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level."]
        #[inline(always)]
        pub const fn rx_iev(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level."]
        #[inline(always)]
        pub fn set_rx_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BlerxtxIevr {
        #[inline(always)]
        fn default() -> BlerxtxIevr {
            BlerxtxIevr(0)
        }
    }
    impl core::fmt::Debug for BlerxtxIevr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlerxtxIevr")
                .field("tx_iev", &self.tx_iev())
                .field("rx_iev", &self.rx_iev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlerxtxIevr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BlerxtxIevr {{ tx_iev: {=bool:?}, rx_iev: {=bool:?} }}",
                self.tx_iev(),
                self.rx_iev()
            )
        }
    }
    #[doc = "BLERXTX_ISCR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlerxtxIscr(pub u8);
    impl BlerxtxIscr {
        #[doc = "TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred."]
        #[inline(always)]
        pub const fn tx_isc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX_ISC:interrupt status on TX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on TX_SEQUENCE detected. 1: activity on TX_SEQUENCE occurred."]
        #[inline(always)]
        pub fn set_tx_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred."]
        #[inline(always)]
        pub const fn rx_isc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX_ISC: interrupt status on RX_SEQUENCE signal (can be a rising or a falling edge depending on BLERXTX_IEVR and BLERXTX_IBER): 0: no activity on RX_SEQUENCE detected. 1: activity on RX_SEQUENCE occurred."]
        #[inline(always)]
        pub fn set_rx_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "TX_ISEDGE: interrupt edge status on TX_SEQUENCE signal: 0: falling edge on TX_SEQUENCE detected. 1: rising edge on TX_SEQUENCE detected."]
        #[inline(always)]
        pub const fn tx_isedge(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TX_ISEDGE: interrupt edge status on TX_SEQUENCE signal: 0: falling edge on TX_SEQUENCE detected. 1: rising edge on TX_SEQUENCE detected."]
        #[inline(always)]
        pub fn set_tx_isedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RX_ISEDGE: interrupt edge status on RX_SEQUENCE signal: 0: falling edge on RX_SEQUENCE detected. 1: rising edge on RX_SEQUENCE detected."]
        #[inline(always)]
        pub const fn rx_isedge(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RX_ISEDGE: interrupt edge status on RX_SEQUENCE signal: 0: falling edge on RX_SEQUENCE detected. 1: rising edge on RX_SEQUENCE detected."]
        #[inline(always)]
        pub fn set_rx_isedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for BlerxtxIscr {
        #[inline(always)]
        fn default() -> BlerxtxIscr {
            BlerxtxIscr(0)
        }
    }
    impl core::fmt::Debug for BlerxtxIscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlerxtxIscr")
                .field("tx_isc", &self.tx_isc())
                .field("rx_isc", &self.rx_isc())
                .field("tx_isedge", &self.tx_isedge())
                .field("rx_isedge", &self.rx_isedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlerxtxIscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BlerxtxIscr {{ tx_isc: {=bool:?}, rx_isc: {=bool:?}, tx_isedge: {=bool:?}, rx_isedge: {=bool:?} }}",
                self.tx_isc(),
                self.rx_isc(),
                self.tx_isedge(),
                self.rx_isedge()
            )
        }
    }
    #[doc = "DIE_ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DieId(pub u32);
    impl DieId {
        #[doc = "Cut revision (metal fix)."]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Cut revision (metal fix)."]
        #[inline(always)]
        pub fn set_revision(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Cut version."]
        #[inline(always)]
        pub const fn version(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Cut version."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Product version. May be used to discriminate several version of a same digital BLE LPH device embedding different analog versions."]
        #[inline(always)]
        pub const fn product(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Product version. May be used to discriminate several version of a same digital BLE LPH device embedding different analog versions."]
        #[inline(always)]
        pub fn set_product(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for DieId {
        #[inline(always)]
        fn default() -> DieId {
            DieId(0)
        }
    }
    impl core::fmt::Debug for DieId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DieId")
                .field("revision", &self.revision())
                .field("version", &self.version())
                .field("product", &self.product())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DieId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DieId {{ revision: {=u8:?}, version: {=u8:?}, product: {=u8:?} }}",
                self.revision(),
                self.version(),
                self.product()
            )
        }
    }
    #[doc = "I2C_FMP_CTRL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2cFmpCtrl(pub u32);
    impl I2cFmpCtrl {
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed."]
        #[inline(always)]
        pub const fn i2c1_pa0_fmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PA0 I/O. 0: PA0 pin operated in standard mode. 1: FM+ mode is enabled on PA0 pin, and speed control is bypassed."]
        #[inline(always)]
        pub fn set_i2c1_pa0_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed."]
        #[inline(always)]
        pub const fn i2c1_pa1_fmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PA1 I/O. 0: PA1 pin operated in standard mode. 1: FM+ mode is enabled on PA1 pin, and speed control is bypassed."]
        #[inline(always)]
        pub fn set_i2c1_pa1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed."]
        #[inline(always)]
        pub const fn i2c1_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SCL on PB6 I/O. 0: PB6 pin operated in standard mode. 1: FM+ mode is enabled on PB6 pin, and speed control is bypassed."]
        #[inline(always)]
        pub fn set_i2c1_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed."]
        #[inline(always)]
        pub const fn i2c1_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-Mode Plus driving capability for I2C1_SDA on PB7 I/O. 0: PB7 pin operated in standard mode. 1: FM+ mode is enabled on PB7 pin, and speed control is bypassed."]
        #[inline(always)]
        pub fn set_i2c1_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for I2cFmpCtrl {
        #[inline(always)]
        fn default() -> I2cFmpCtrl {
            I2cFmpCtrl(0)
        }
    }
    impl core::fmt::Debug for I2cFmpCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2cFmpCtrl")
                .field("i2c1_pa0_fmp", &self.i2c1_pa0_fmp())
                .field("i2c1_pa1_fmp", &self.i2c1_pa1_fmp())
                .field("i2c1_pb6_fmp", &self.i2c1_pb6_fmp())
                .field("i2c1_pb7_fmp", &self.i2c1_pb7_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2cFmpCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "I2cFmpCtrl {{ i2c1_pa0_fmp: {=bool:?}, i2c1_pa1_fmp: {=bool:?}, i2c1_pb6_fmp: {=bool:?}, i2c1_pb7_fmp: {=bool:?} }}" , self . i2c1_pa0_fmp () , self . i2c1_pa1_fmp () , self . i2c1_pb6_fmp () , self . i2c1_pb7_fmp ())
        }
    }
    #[doc = "IO_DTR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoDtr(pub u32);
    impl IoDtr {
        #[doc = "PA0_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa0_dt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA0_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa0_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PA1_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa1_dt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA1_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa1_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PA2_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa2_dt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA2_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa2_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PA3_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa3_dt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa3_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA4_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub const fn pa4_dt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA4_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub fn set_pa4_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PA5_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub const fn pa5_dt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PA5_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub fn set_pa5_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PA6_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub const fn pa6_dt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PA6_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub fn set_pa6_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PA7_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub const fn pa7_dt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PA7_DT:Interrupt Detection Type for port A I/Os."]
        #[inline(always)]
        pub fn set_pa7_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PA8_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa8_dt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PA8_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa8_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PA9_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa9_dt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PA9_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa9_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA10_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa10_dt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PA10_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa10_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PA11_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pa11_dt(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PA11_DT: Interrupt Detection Type for port A I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pa11_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PB0_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb0_dt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB0_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb0_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB1_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb1_dt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB1_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb1_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB2_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb2_dt(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB2_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb2_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB3_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb3_dt(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB3_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb3_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PB4_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb4_dt(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PB4_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb4_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PB5_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb5_dt(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PB5_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb5_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PB6_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb6_dt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb6_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PB7_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb7_dt(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb7_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PB8_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub const fn pb8_dt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub fn set_pb8_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PB9_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub const fn pb9_dt(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PB9_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub fn set_pb9_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PB10_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub const fn pb10_dt(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PB10_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub fn set_pb10_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PB11_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub const fn pb11_dt(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PB11_DT:Interrupt Detection Type for port B I/Os."]
        #[inline(always)]
        pub fn set_pb11_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PB12_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb12_dt(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PB12_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb12_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PB13_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb13_dt(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PB13_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb13_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PB14_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb14_dt(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PB14_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb14_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "PB15_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub const fn pb15_dt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PB15_DT: Interrupt Detection Type for port B I/Os. 0: edge detection. 1: level detection."]
        #[inline(always)]
        pub fn set_pb15_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IoDtr {
        #[inline(always)]
        fn default() -> IoDtr {
            IoDtr(0)
        }
    }
    impl core::fmt::Debug for IoDtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoDtr")
                .field("pa0_dt", &self.pa0_dt())
                .field("pa1_dt", &self.pa1_dt())
                .field("pa2_dt", &self.pa2_dt())
                .field("pa3_dt", &self.pa3_dt())
                .field("pa4_dt", &self.pa4_dt())
                .field("pa5_dt", &self.pa5_dt())
                .field("pa6_dt", &self.pa6_dt())
                .field("pa7_dt", &self.pa7_dt())
                .field("pa8_dt", &self.pa8_dt())
                .field("pa9_dt", &self.pa9_dt())
                .field("pa10_dt", &self.pa10_dt())
                .field("pa11_dt", &self.pa11_dt())
                .field("pb0_dt", &self.pb0_dt())
                .field("pb1_dt", &self.pb1_dt())
                .field("pb2_dt", &self.pb2_dt())
                .field("pb3_dt", &self.pb3_dt())
                .field("pb4_dt", &self.pb4_dt())
                .field("pb5_dt", &self.pb5_dt())
                .field("pb6_dt", &self.pb6_dt())
                .field("pb7_dt", &self.pb7_dt())
                .field("pb8_dt", &self.pb8_dt())
                .field("pb9_dt", &self.pb9_dt())
                .field("pb10_dt", &self.pb10_dt())
                .field("pb11_dt", &self.pb11_dt())
                .field("pb12_dt", &self.pb12_dt())
                .field("pb13_dt", &self.pb13_dt())
                .field("pb14_dt", &self.pb14_dt())
                .field("pb15_dt", &self.pb15_dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoDtr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IoDtr {{ pa0_dt: {=bool:?}, pa1_dt: {=bool:?}, pa2_dt: {=bool:?}, pa3_dt: {=bool:?}, pa4_dt: {=bool:?}, pa5_dt: {=bool:?}, pa6_dt: {=bool:?}, pa7_dt: {=bool:?}, pa8_dt: {=bool:?}, pa9_dt: {=bool:?}, pa10_dt: {=bool:?}, pa11_dt: {=bool:?}, pb0_dt: {=bool:?}, pb1_dt: {=bool:?}, pb2_dt: {=bool:?}, pb3_dt: {=bool:?}, pb4_dt: {=bool:?}, pb5_dt: {=bool:?}, pb6_dt: {=bool:?}, pb7_dt: {=bool:?}, pb8_dt: {=bool:?}, pb9_dt: {=bool:?}, pb10_dt: {=bool:?}, pb11_dt: {=bool:?}, pb12_dt: {=bool:?}, pb13_dt: {=bool:?}, pb14_dt: {=bool:?}, pb15_dt: {=bool:?} }}" , self . pa0_dt () , self . pa1_dt () , self . pa2_dt () , self . pa3_dt () , self . pa4_dt () , self . pa5_dt () , self . pa6_dt () , self . pa7_dt () , self . pa8_dt () , self . pa9_dt () , self . pa10_dt () , self . pa11_dt () , self . pb0_dt () , self . pb1_dt () , self . pb2_dt () , self . pb3_dt () , self . pb4_dt () , self . pb5_dt () , self . pb6_dt () , self . pb7_dt () , self . pb8_dt () , self . pb9_dt () , self . pb10_dt () , self . pb11_dt () , self . pb12_dt () , self . pb13_dt () , self . pb14_dt () , self . pb15_dt ())
        }
    }
    #[doc = "IO_IBER register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoIber(pub u32);
    impl IoIber {
        #[doc = "PA0_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa0_ibe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA0_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa0_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PA1_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa1_ibe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA1_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa1_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PA2_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa2_ibe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA2_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa2_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PA3_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa3_ibe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa3_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA4_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa4_ibe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA4_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa4_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PA5_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa5_ibe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PA5_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa5_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PA6_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa6_ibe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PA6_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa6_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PA7_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa7_ibe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PA7_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa7_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PA8_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa8_ibe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PA8_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa8_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PA9_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa9_ibe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PA9_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa9_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA10_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa10_ibe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PA10_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa10_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PA11_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pa11_ibe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PA11_IBE: Interrupt edge selection for Port A I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pa11_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PA12_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa12_ibe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PA12_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa12_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PA13_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa13_ibe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PA13_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa13_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PA14_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa14_ibe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PA14_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa14_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PA15_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub const fn pa15_ibe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PA15_IBE: Interrupt edge selection for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa15_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PB0_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb0_ibe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB0_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb0_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB1_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb1_ibe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB1_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb1_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB2_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb2_ibe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB2_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb2_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB3_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb3_ibe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB3_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb3_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PB4_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb4_ibe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PB4_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb4_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PB5_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb5_ibe(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PB5_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb5_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PB6_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb6_ibe(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb6_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PB7_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb7_ibe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb7_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PB8_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub const fn pb8_ibe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub fn set_pb8_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PB9_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub const fn pb9_ibe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PB9_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub fn set_pb9_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PB10_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub const fn pb10_ibe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PB10_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub fn set_pb10_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PB11_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub const fn pb11_ibe(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PB11_IBE: Interrupt edge selection for port B I/Os."]
        #[inline(always)]
        pub fn set_pb11_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PB12_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb12_ibe(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PB12_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb12_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PB13_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb13_ibe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PB13_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb13_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PB14_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb14_ibe(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PB14_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb14_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "PB15_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub const fn pb15_ibe(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PB15_IBE: Interrupt edge selection for port B I/Os. 0: single edge detection. 1: both edges detection."]
        #[inline(always)]
        pub fn set_pb15_ibe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IoIber {
        #[inline(always)]
        fn default() -> IoIber {
            IoIber(0)
        }
    }
    impl core::fmt::Debug for IoIber {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoIber")
                .field("pa0_ibe", &self.pa0_ibe())
                .field("pa1_ibe", &self.pa1_ibe())
                .field("pa2_ibe", &self.pa2_ibe())
                .field("pa3_ibe", &self.pa3_ibe())
                .field("pa4_ibe", &self.pa4_ibe())
                .field("pa5_ibe", &self.pa5_ibe())
                .field("pa6_ibe", &self.pa6_ibe())
                .field("pa7_ibe", &self.pa7_ibe())
                .field("pa8_ibe", &self.pa8_ibe())
                .field("pa9_ibe", &self.pa9_ibe())
                .field("pa10_ibe", &self.pa10_ibe())
                .field("pa11_ibe", &self.pa11_ibe())
                .field("pa12_ibe", &self.pa12_ibe())
                .field("pa13_ibe", &self.pa13_ibe())
                .field("pa14_ibe", &self.pa14_ibe())
                .field("pa15_ibe", &self.pa15_ibe())
                .field("pb0_ibe", &self.pb0_ibe())
                .field("pb1_ibe", &self.pb1_ibe())
                .field("pb2_ibe", &self.pb2_ibe())
                .field("pb3_ibe", &self.pb3_ibe())
                .field("pb4_ibe", &self.pb4_ibe())
                .field("pb5_ibe", &self.pb5_ibe())
                .field("pb6_ibe", &self.pb6_ibe())
                .field("pb7_ibe", &self.pb7_ibe())
                .field("pb8_ibe", &self.pb8_ibe())
                .field("pb9_ibe", &self.pb9_ibe())
                .field("pb10_ibe", &self.pb10_ibe())
                .field("pb11_ibe", &self.pb11_ibe())
                .field("pb12_ibe", &self.pb12_ibe())
                .field("pb13_ibe", &self.pb13_ibe())
                .field("pb14_ibe", &self.pb14_ibe())
                .field("pb15_ibe", &self.pb15_ibe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoIber {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IoIber {{ pa0_ibe: {=bool:?}, pa1_ibe: {=bool:?}, pa2_ibe: {=bool:?}, pa3_ibe: {=bool:?}, pa4_ibe: {=bool:?}, pa5_ibe: {=bool:?}, pa6_ibe: {=bool:?}, pa7_ibe: {=bool:?}, pa8_ibe: {=bool:?}, pa9_ibe: {=bool:?}, pa10_ibe: {=bool:?}, pa11_ibe: {=bool:?}, pa12_ibe: {=bool:?}, pa13_ibe: {=bool:?}, pa14_ibe: {=bool:?}, pa15_ibe: {=bool:?}, pb0_ibe: {=bool:?}, pb1_ibe: {=bool:?}, pb2_ibe: {=bool:?}, pb3_ibe: {=bool:?}, pb4_ibe: {=bool:?}, pb5_ibe: {=bool:?}, pb6_ibe: {=bool:?}, pb7_ibe: {=bool:?}, pb8_ibe: {=bool:?}, pb9_ibe: {=bool:?}, pb10_ibe: {=bool:?}, pb11_ibe: {=bool:?}, pb12_ibe: {=bool:?}, pb13_ibe: {=bool:?}, pb14_ibe: {=bool:?}, pb15_ibe: {=bool:?} }}" , self . pa0_ibe () , self . pa1_ibe () , self . pa2_ibe () , self . pa3_ibe () , self . pa4_ibe () , self . pa5_ibe () , self . pa6_ibe () , self . pa7_ibe () , self . pa8_ibe () , self . pa9_ibe () , self . pa10_ibe () , self . pa11_ibe () , self . pa12_ibe () , self . pa13_ibe () , self . pa14_ibe () , self . pa15_ibe () , self . pb0_ibe () , self . pb1_ibe () , self . pb2_ibe () , self . pb3_ibe () , self . pb4_ibe () , self . pb5_ibe () , self . pb6_ibe () , self . pb7_ibe () , self . pb8_ibe () , self . pb9_ibe () , self . pb10_ibe () , self . pb11_ibe () , self . pb12_ibe () , self . pb13_ibe () , self . pb14_ibe () , self . pb15_ibe ())
        }
    }
    #[doc = "IO_IER register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoIer(pub u32);
    impl IoIer {
        #[doc = "PA0_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa0_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA0_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa0_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PA1_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa1_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA1_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa1_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PA2_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa2_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA2_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa2_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PA3_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa3_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa3_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA4_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa4_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA4_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa4_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PA5_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa5_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PA5_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa5_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PA6_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa6_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PA6_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa6_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PA7_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa7_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PA7_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa7_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PA8_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa8_ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PA8_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa8_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PA9_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa9_ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PA9_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa9_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA10_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa10_ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PA10_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa10_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PA11_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pa11_ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PA11_IE: Interrupt enable for port A I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pa11_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PA12_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa12_ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PA12_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa12_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PA13_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa13_ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PA13_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa13_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PA14_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa14_ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PA14_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa14_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PA15_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub const fn pa15_ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PA15_IE: Interrupt enable for port A I/Os."]
        #[inline(always)]
        pub fn set_pa15_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PB0_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb0_ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB0_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb0_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB1_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb1_ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB1_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb1_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB2_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb2_ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB2_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb2_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB3_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb3_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB3_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb3_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PB4_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb4_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PB4_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb4_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PB5_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb5_ie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PB5_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb5_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PB6_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb6_ie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb6_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PB7_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb7_ie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb7_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PB8_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub const fn pb8_ie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub fn set_pb8_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PB9_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub const fn pb9_ie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PB9_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub fn set_pb9_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PB10_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub const fn pb10_ie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PB10_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub fn set_pb10_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PB11_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub const fn pb11_ie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PB11_IE: Interrupt enable for port B I/Os."]
        #[inline(always)]
        pub fn set_pb11_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PB12_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb12_ie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PB12_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb12_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PB13_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb13_ie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PB13_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb13_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PB14_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb14_ie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PB14_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb14_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "PB15_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub const fn pb15_ie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PB15_IE: Interrupt enable for port B I/Os. 0: interrupt is disabled. 1: interrupt is enabled."]
        #[inline(always)]
        pub fn set_pb15_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IoIer {
        #[inline(always)]
        fn default() -> IoIer {
            IoIer(0)
        }
    }
    impl core::fmt::Debug for IoIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoIer")
                .field("pa0_ie", &self.pa0_ie())
                .field("pa1_ie", &self.pa1_ie())
                .field("pa2_ie", &self.pa2_ie())
                .field("pa3_ie", &self.pa3_ie())
                .field("pa4_ie", &self.pa4_ie())
                .field("pa5_ie", &self.pa5_ie())
                .field("pa6_ie", &self.pa6_ie())
                .field("pa7_ie", &self.pa7_ie())
                .field("pa8_ie", &self.pa8_ie())
                .field("pa9_ie", &self.pa9_ie())
                .field("pa10_ie", &self.pa10_ie())
                .field("pa11_ie", &self.pa11_ie())
                .field("pa12_ie", &self.pa12_ie())
                .field("pa13_ie", &self.pa13_ie())
                .field("pa14_ie", &self.pa14_ie())
                .field("pa15_ie", &self.pa15_ie())
                .field("pb0_ie", &self.pb0_ie())
                .field("pb1_ie", &self.pb1_ie())
                .field("pb2_ie", &self.pb2_ie())
                .field("pb3_ie", &self.pb3_ie())
                .field("pb4_ie", &self.pb4_ie())
                .field("pb5_ie", &self.pb5_ie())
                .field("pb6_ie", &self.pb6_ie())
                .field("pb7_ie", &self.pb7_ie())
                .field("pb8_ie", &self.pb8_ie())
                .field("pb9_ie", &self.pb9_ie())
                .field("pb10_ie", &self.pb10_ie())
                .field("pb11_ie", &self.pb11_ie())
                .field("pb12_ie", &self.pb12_ie())
                .field("pb13_ie", &self.pb13_ie())
                .field("pb14_ie", &self.pb14_ie())
                .field("pb15_ie", &self.pb15_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoIer {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IoIer {{ pa0_ie: {=bool:?}, pa1_ie: {=bool:?}, pa2_ie: {=bool:?}, pa3_ie: {=bool:?}, pa4_ie: {=bool:?}, pa5_ie: {=bool:?}, pa6_ie: {=bool:?}, pa7_ie: {=bool:?}, pa8_ie: {=bool:?}, pa9_ie: {=bool:?}, pa10_ie: {=bool:?}, pa11_ie: {=bool:?}, pa12_ie: {=bool:?}, pa13_ie: {=bool:?}, pa14_ie: {=bool:?}, pa15_ie: {=bool:?}, pb0_ie: {=bool:?}, pb1_ie: {=bool:?}, pb2_ie: {=bool:?}, pb3_ie: {=bool:?}, pb4_ie: {=bool:?}, pb5_ie: {=bool:?}, pb6_ie: {=bool:?}, pb7_ie: {=bool:?}, pb8_ie: {=bool:?}, pb9_ie: {=bool:?}, pb10_ie: {=bool:?}, pb11_ie: {=bool:?}, pb12_ie: {=bool:?}, pb13_ie: {=bool:?}, pb14_ie: {=bool:?}, pb15_ie: {=bool:?} }}" , self . pa0_ie () , self . pa1_ie () , self . pa2_ie () , self . pa3_ie () , self . pa4_ie () , self . pa5_ie () , self . pa6_ie () , self . pa7_ie () , self . pa8_ie () , self . pa9_ie () , self . pa10_ie () , self . pa11_ie () , self . pa12_ie () , self . pa13_ie () , self . pa14_ie () , self . pa15_ie () , self . pb0_ie () , self . pb1_ie () , self . pb2_ie () , self . pb3_ie () , self . pb4_ie () , self . pb5_ie () , self . pb6_ie () , self . pb7_ie () , self . pb8_ie () , self . pb9_ie () , self . pb10_ie () , self . pb11_ie () , self . pb12_ie () , self . pb13_ie () , self . pb14_ie () , self . pb15_ie ())
        }
    }
    #[doc = "IO_IEVR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoIevr(pub u32);
    impl IoIevr {
        #[doc = "PA0_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa0_iev(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA0_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa0_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PA1_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa1_iev(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA1_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa1_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PA2_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa2_iev(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA2_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa2_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PA3_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa3_iev(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa3_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA4_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa4_iev(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA4_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa4_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PA5_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa5_iev(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PA5_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa5_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PA6_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa6_iev(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PA6_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa6_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PA7_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa7_iev(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PA7_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa7_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PA8_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa8_iev(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PA8_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa8_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PA9_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa9_iev(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PA9_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa9_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA10_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa10_iev(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PA10_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa10_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PA11_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pa11_iev(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PA11_IEV : Interrupt polarity event for Port A I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pa11_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PA12_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa12_iev(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PA12_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa12_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PA13_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa13_iev(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PA13_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa13_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PA14_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa14_iev(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PA14_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa14_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PA15_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub const fn pa15_iev(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PA15_IEV : Interrupt polarity event for Port A I/Os."]
        #[inline(always)]
        pub fn set_pa15_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PB0_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb0_iev(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB0_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb0_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB1_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb1_iev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB1_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb1_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB2_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb2_iev(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB2_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb2_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB3_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb3_iev(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB3_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb3_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PB4_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb4_iev(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PB4_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb4_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PB5_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb5_iev(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PB5_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb5_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PB6_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb6_iev(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb6_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PB7_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb7_iev(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb7_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PB8_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub const fn pb8_iev(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub fn set_pb8_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PB9_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub const fn pb9_iev(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PB9_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub fn set_pb9_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PB10_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub const fn pb10_iev(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PB10_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub fn set_pb10_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PB11_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub const fn pb11_iev(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PB11_IEV : Interrupt polarity event for Port B I/Os."]
        #[inline(always)]
        pub fn set_pb11_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PB12_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb12_iev(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PB12_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb12_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PB13_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb13_iev(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PB13_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb13_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PB14_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb14_iev(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PB14_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb14_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "PB15_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub const fn pb15_iev(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PB15_IEV : Interrupt polarity event for Port B I/Os. 0: falling edge / low level. 1: rising edge / high level."]
        #[inline(always)]
        pub fn set_pb15_iev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IoIevr {
        #[inline(always)]
        fn default() -> IoIevr {
            IoIevr(0)
        }
    }
    impl core::fmt::Debug for IoIevr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoIevr")
                .field("pa0_iev", &self.pa0_iev())
                .field("pa1_iev", &self.pa1_iev())
                .field("pa2_iev", &self.pa2_iev())
                .field("pa3_iev", &self.pa3_iev())
                .field("pa4_iev", &self.pa4_iev())
                .field("pa5_iev", &self.pa5_iev())
                .field("pa6_iev", &self.pa6_iev())
                .field("pa7_iev", &self.pa7_iev())
                .field("pa8_iev", &self.pa8_iev())
                .field("pa9_iev", &self.pa9_iev())
                .field("pa10_iev", &self.pa10_iev())
                .field("pa11_iev", &self.pa11_iev())
                .field("pa12_iev", &self.pa12_iev())
                .field("pa13_iev", &self.pa13_iev())
                .field("pa14_iev", &self.pa14_iev())
                .field("pa15_iev", &self.pa15_iev())
                .field("pb0_iev", &self.pb0_iev())
                .field("pb1_iev", &self.pb1_iev())
                .field("pb2_iev", &self.pb2_iev())
                .field("pb3_iev", &self.pb3_iev())
                .field("pb4_iev", &self.pb4_iev())
                .field("pb5_iev", &self.pb5_iev())
                .field("pb6_iev", &self.pb6_iev())
                .field("pb7_iev", &self.pb7_iev())
                .field("pb8_iev", &self.pb8_iev())
                .field("pb9_iev", &self.pb9_iev())
                .field("pb10_iev", &self.pb10_iev())
                .field("pb11_iev", &self.pb11_iev())
                .field("pb12_iev", &self.pb12_iev())
                .field("pb13_iev", &self.pb13_iev())
                .field("pb14_iev", &self.pb14_iev())
                .field("pb15_iev", &self.pb15_iev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoIevr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IoIevr {{ pa0_iev: {=bool:?}, pa1_iev: {=bool:?}, pa2_iev: {=bool:?}, pa3_iev: {=bool:?}, pa4_iev: {=bool:?}, pa5_iev: {=bool:?}, pa6_iev: {=bool:?}, pa7_iev: {=bool:?}, pa8_iev: {=bool:?}, pa9_iev: {=bool:?}, pa10_iev: {=bool:?}, pa11_iev: {=bool:?}, pa12_iev: {=bool:?}, pa13_iev: {=bool:?}, pa14_iev: {=bool:?}, pa15_iev: {=bool:?}, pb0_iev: {=bool:?}, pb1_iev: {=bool:?}, pb2_iev: {=bool:?}, pb3_iev: {=bool:?}, pb4_iev: {=bool:?}, pb5_iev: {=bool:?}, pb6_iev: {=bool:?}, pb7_iev: {=bool:?}, pb8_iev: {=bool:?}, pb9_iev: {=bool:?}, pb10_iev: {=bool:?}, pb11_iev: {=bool:?}, pb12_iev: {=bool:?}, pb13_iev: {=bool:?}, pb14_iev: {=bool:?}, pb15_iev: {=bool:?} }}" , self . pa0_iev () , self . pa1_iev () , self . pa2_iev () , self . pa3_iev () , self . pa4_iev () , self . pa5_iev () , self . pa6_iev () , self . pa7_iev () , self . pa8_iev () , self . pa9_iev () , self . pa10_iev () , self . pa11_iev () , self . pa12_iev () , self . pa13_iev () , self . pa14_iev () , self . pa15_iev () , self . pb0_iev () , self . pb1_iev () , self . pb2_iev () , self . pb3_iev () , self . pb4_iev () , self . pb5_iev () , self . pb6_iev () , self . pb7_iev () , self . pb8_iev () , self . pb9_iev () , self . pb10_iev () , self . pb11_iev () , self . pb12_iev () , self . pb13_iev () , self . pb14_iev () , self . pb15_iev ())
        }
    }
    #[doc = "IO_ISCR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoIscr(pub u32);
    impl IoIscr {
        #[doc = "PA0_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa0_isc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA0_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa0_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PA1_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa1_isc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA1_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa1_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PA2_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa2_isc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA2_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa2_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PA3_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa3_isc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA3_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa3_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA4_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub const fn pa4_isc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA4_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub fn set_pa4_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PA5_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub const fn pa5_isc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PA5_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub fn set_pa5_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PA6_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub const fn pa6_isc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PA6_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub fn set_pa6_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PA7_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub const fn pa7_isc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PA7_ISC: Interrupt status (before mask) for port a I/Os.."]
        #[inline(always)]
        pub fn set_pa7_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PA8_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa8_isc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PA8_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa8_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PA9_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa9_isc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PA9_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa9_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PA10_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa10_isc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PA10_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa10_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PA11_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pa11_isc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PA11_ISC: Interrupt status (before mask) for port a I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pa11_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PA12_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub const fn pa12_isc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PA12_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub fn set_pa12_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PA13_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub const fn pa13_isc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PA13_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub fn set_pa13_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PA14_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub const fn pa14_isc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PA14_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub fn set_pa14_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PA15_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub const fn pa15_isc(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PA15_ISC: Interrupt status (before mask) for port a I/Os."]
        #[inline(always)]
        pub fn set_pa15_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PB0_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb0_isc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PB0_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb0_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PB1_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb1_isc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PB1_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb1_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PB2_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb2_isc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PB2_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb2_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PB3_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb3_isc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PB3_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb3_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PB4_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb4_isc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PB4_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb4_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PB5_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb5_isc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PB5_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb5_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PB6_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb6_isc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PB6_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb6_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PB7_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb7_isc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "PB7_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb7_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PB8_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub const fn pb8_isc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PB8_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub fn set_pb8_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PB9_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub const fn pb9_isc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PB9_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub fn set_pb9_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PB10_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub const fn pb10_isc(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PB10_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub fn set_pb10_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PB11_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub const fn pb11_isc(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PB11_ISC: Interrupt status (before mask) for port B I/Os.."]
        #[inline(always)]
        pub fn set_pb11_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PB12_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb12_isc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PB12_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb12_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PB13_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb13_isc(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PB13_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb13_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PB14_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb14_isc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PB14_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb14_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "PB15_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pb15_isc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PB15_ISC: Interrupt status (before mask) for port B I/Os. 0: no pending interrupt. 1: event occurred on corresponding I/O / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pb15_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IoIscr {
        #[inline(always)]
        fn default() -> IoIscr {
            IoIscr(0)
        }
    }
    impl core::fmt::Debug for IoIscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoIscr")
                .field("pa0_isc", &self.pa0_isc())
                .field("pa1_isc", &self.pa1_isc())
                .field("pa2_isc", &self.pa2_isc())
                .field("pa3_isc", &self.pa3_isc())
                .field("pa4_isc", &self.pa4_isc())
                .field("pa5_isc", &self.pa5_isc())
                .field("pa6_isc", &self.pa6_isc())
                .field("pa7_isc", &self.pa7_isc())
                .field("pa8_isc", &self.pa8_isc())
                .field("pa9_isc", &self.pa9_isc())
                .field("pa10_isc", &self.pa10_isc())
                .field("pa11_isc", &self.pa11_isc())
                .field("pa12_isc", &self.pa12_isc())
                .field("pa13_isc", &self.pa13_isc())
                .field("pa14_isc", &self.pa14_isc())
                .field("pa15_isc", &self.pa15_isc())
                .field("pb0_isc", &self.pb0_isc())
                .field("pb1_isc", &self.pb1_isc())
                .field("pb2_isc", &self.pb2_isc())
                .field("pb3_isc", &self.pb3_isc())
                .field("pb4_isc", &self.pb4_isc())
                .field("pb5_isc", &self.pb5_isc())
                .field("pb6_isc", &self.pb6_isc())
                .field("pb7_isc", &self.pb7_isc())
                .field("pb8_isc", &self.pb8_isc())
                .field("pb9_isc", &self.pb9_isc())
                .field("pb10_isc", &self.pb10_isc())
                .field("pb11_isc", &self.pb11_isc())
                .field("pb12_isc", &self.pb12_isc())
                .field("pb13_isc", &self.pb13_isc())
                .field("pb14_isc", &self.pb14_isc())
                .field("pb15_isc", &self.pb15_isc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoIscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IoIscr {{ pa0_isc: {=bool:?}, pa1_isc: {=bool:?}, pa2_isc: {=bool:?}, pa3_isc: {=bool:?}, pa4_isc: {=bool:?}, pa5_isc: {=bool:?}, pa6_isc: {=bool:?}, pa7_isc: {=bool:?}, pa8_isc: {=bool:?}, pa9_isc: {=bool:?}, pa10_isc: {=bool:?}, pa11_isc: {=bool:?}, pa12_isc: {=bool:?}, pa13_isc: {=bool:?}, pa14_isc: {=bool:?}, pa15_isc: {=bool:?}, pb0_isc: {=bool:?}, pb1_isc: {=bool:?}, pb2_isc: {=bool:?}, pb3_isc: {=bool:?}, pb4_isc: {=bool:?}, pb5_isc: {=bool:?}, pb6_isc: {=bool:?}, pb7_isc: {=bool:?}, pb8_isc: {=bool:?}, pb9_isc: {=bool:?}, pb10_isc: {=bool:?}, pb11_isc: {=bool:?}, pb12_isc: {=bool:?}, pb13_isc: {=bool:?}, pb14_isc: {=bool:?}, pb15_isc: {=bool:?} }}" , self . pa0_isc () , self . pa1_isc () , self . pa2_isc () , self . pa3_isc () , self . pa4_isc () , self . pa5_isc () , self . pa6_isc () , self . pa7_isc () , self . pa8_isc () , self . pa9_isc () , self . pa10_isc () , self . pa11_isc () , self . pa12_isc () , self . pa13_isc () , self . pa14_isc () , self . pa15_isc () , self . pb0_isc () , self . pb1_isc () , self . pb2_isc () , self . pb3_isc () , self . pb4_isc () , self . pb5_isc () , self . pb6_isc () , self . pb7_isc () , self . pb8_isc () , self . pb9_isc () , self . pb10_isc () , self . pb11_isc () , self . pb12_isc () , self . pb13_isc () , self . pb14_isc () , self . pb15_isc ())
        }
    }
    #[doc = "JTAG_ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JtagId(pub u32);
    impl JtagId {
        #[doc = "Manufacturer ID."]
        #[inline(always)]
        pub const fn manuf_id(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x07ff;
            val as u16
        }
        #[doc = "Manufacturer ID."]
        #[inline(always)]
        pub fn set_manuf_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 1usize)) | (((val as u32) & 0x07ff) << 1usize);
        }
        #[doc = "Part number."]
        #[inline(always)]
        pub const fn part_number(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Part number."]
        #[inline(always)]
        pub fn set_part_number(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "Version."]
        #[inline(always)]
        pub const fn version_number(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Version."]
        #[inline(always)]
        pub fn set_version_number(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for JtagId {
        #[inline(always)]
        fn default() -> JtagId {
            JtagId(0)
        }
    }
    impl core::fmt::Debug for JtagId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JtagId")
                .field("manuf_id", &self.manuf_id())
                .field("part_number", &self.part_number())
                .field("version_number", &self.version_number())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JtagId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JtagId {{ manuf_id: {=u16:?}, part_number: {=u16:?}, version_number: {=u8:?} }}",
                self.manuf_id(),
                self.part_number(),
                self.version_number()
            )
        }
    }
    #[doc = "PWRC_IER register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwrcIer(pub u32);
    impl PwrcIer {
        #[doc = "BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled."]
        #[inline(always)]
        pub const fn borh_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BORH_IE: BORH interrupt enable. 0: BORH interrupt is disabled. 1: BORH interrupt is enabled."]
        #[inline(always)]
        pub fn set_borh_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled."]
        #[inline(always)]
        pub const fn pvd_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PVD_IE: Programmable Voltage Detector interrupt enable. 0: PVD interrupt is disabled. 1: PVD interrupt is enabled."]
        #[inline(always)]
        pub fn set_pvd_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled."]
        #[inline(always)]
        pub const fn wkup_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WKUP_IE: Power Controller Wakeup event interrupt enable. 0: Interrupt on wakeup event seen by the PWRC is disabled. 1: Interrupt on wakeup event seen by the PWRC is enabled."]
        #[inline(always)]
        pub fn set_wkup_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for PwrcIer {
        #[inline(always)]
        fn default() -> PwrcIer {
            PwrcIer(0)
        }
    }
    impl core::fmt::Debug for PwrcIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwrcIer")
                .field("borh_ie", &self.borh_ie())
                .field("pvd_ie", &self.pvd_ie())
                .field("wkup_ie", &self.wkup_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwrcIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwrcIer {{ borh_ie: {=bool:?}, pvd_ie: {=bool:?}, wkup_ie: {=bool:?} }}",
                self.borh_ie(),
                self.pvd_ie(),
                self.wkup_ie()
            )
        }
    }
    #[doc = "PWRC_ISCR register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwrcIscr(pub u32);
    impl PwrcIscr {
        #[doc = "BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn borh_isc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BORH_ISC: BORH interrupt status. 0: no pending interrupt. 1: voltage went under BORH threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_borh_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub const fn pvd_isc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PVD_ISC: Programmable Voltage Detector status. 0: no pending interrupt. 1: voltage went under programmed threshold / interrupt occurred (if enabled). Cleared by writing 1 in the bit."]
        #[inline(always)]
        pub fn set_pvd_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system."]
        #[inline(always)]
        pub const fn wkup_isc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WKUP_ISC: Indicates the Power Controller receives a Wakeup event. 0: no pending interrupt. 1: Wakeup event on PWRC occurred / interrupt occurred (if enabled). Cleared by writing 1 in the bit. This flag will be read at 1 if a wakeup event arrives so close to the low power mode entry requests that the PWRC aborts before shutting down the system."]
        #[inline(always)]
        pub fn set_wkup_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for PwrcIscr {
        #[inline(always)]
        fn default() -> PwrcIscr {
            PwrcIscr(0)
        }
    }
    impl core::fmt::Debug for PwrcIscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwrcIscr")
                .field("borh_isc", &self.borh_isc())
                .field("pvd_isc", &self.pvd_isc())
                .field("wkup_isc", &self.wkup_isc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwrcIscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwrcIscr {{ borh_isc: {=bool:?}, pvd_isc: {=bool:?}, wkup_isc: {=bool:?} }}",
                self.borh_isc(),
                self.pvd_isc(),
                self.wkup_isc()
            )
        }
    }
}
