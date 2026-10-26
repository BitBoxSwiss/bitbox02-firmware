#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Parallel synchronous slave interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pssi {
    ptr: *mut u8,
}
unsafe impl Send for Pssi {}
unsafe impl Sync for Pssi {}
impl Pssi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PSSI control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PSSI status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PSSI raw interrupt status register."]
    #[inline(always)]
    pub const fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PSSI interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PSSI masked interrupt status register."]
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PSSI interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PSSI data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "PSSI control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN."]
        #[inline(always)]
        pub const fn ckpol(&self) -> super::vals::Ckpol {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Ckpol::from_bits(val as u8)
        }
        #[doc = "Parallel data clock polarity This bit configures the capture edge of the parallel clock or the edge used for driving outputs, depending on OUTEN."]
        #[inline(always)]
        pub fn set_ckpol(&mut self, val: super::vals::Ckpol) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface."]
        #[inline(always)]
        pub const fn depol(&self) -> super::vals::Depol {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Depol::from_bits(val as u8)
        }
        #[doc = "Data enable (PSSI_DE) polarity This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel interface."]
        #[inline(always)]
        pub fn set_depol(&mut self, val: super::vals::Depol) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface."]
        #[inline(always)]
        pub const fn rdypol(&self) -> super::vals::Rdypol {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Rdypol::from_bits(val as u8)
        }
        #[doc = "Ready (PSSI_RDY) polarity This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel interface."]
        #[inline(always)]
        pub fn set_rdypol(&mut self, val: super::vals::Rdypol) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Extended data mode."]
        #[inline(always)]
        pub const fn edm(&self) -> super::vals::Edm {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Edm::from_bits(val as u8)
        }
        #[doc = "Extended data mode."]
        #[inline(always)]
        pub fn set_edm(&mut self, val: super::vals::Edm) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PSSI enable The contents of the FIFO are flushed when ENABLE is cleared to 0. Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from 0 to 1. The DMA controller and all PSSI configuration registers must be programmed correctly before setting the ENABLE bit to 1. The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at the same time."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity."]
        #[inline(always)]
        pub const fn derdycfg(&self) -> super::vals::Derdycfg {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::Derdycfg::from_bits(val as u8)
        }
        #[doc = "Data enable and ready configuration When the PSSI_RDY function is mapped to the PSSI_DE pin (settings 101 or 111), it is still the RDYPOL bit which determines its polarity. Similarly, when the PSSI_DE function is mapped to the PSSI_RDY pin (settings 110 or 111), it is still the DEPOL bit which determines its polarity."]
        #[inline(always)]
        pub fn set_derdycfg(&mut self, val: super::vals::Derdycfg) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "DMA enable bit."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable bit."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Data direction selection bit."]
        #[inline(always)]
        pub const fn outen(&self) -> super::vals::Outen {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Outen::from_bits(val as u8)
        }
        #[doc = "Data direction selection bit."]
        #[inline(always)]
        pub fn set_outen(&mut self, val: super::vals::Outen) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("ckpol", &self.ckpol())
                .field("depol", &self.depol())
                .field("rdypol", &self.rdypol())
                .field("edm", &self.edm())
                .field("enable", &self.enable())
                .field("derdycfg", &self.derdycfg())
                .field("dmaen", &self.dmaen())
                .field("outen", &self.outen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ ckpol: {:?}, depol: {:?}, rdypol: {:?}, edm: {:?}, enable: {=bool:?}, derdycfg: {:?}, dmaen: {=bool:?}, outen: {:?} }}" , self . ckpol () , self . depol () , self . rdypol () , self . edm () , self . enable () , self . derdycfg () , self . dmaen () , self . outen ())
        }
    }
    #[doc = "PSSI data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub const fn byte(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "Data byte 0."]
        #[inline(always)]
        pub fn set_byte(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
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
            f.debug_struct("Dr")
                .field("byte[0]", &self.byte(0usize))
                .field("byte[1]", &self.byte(1usize))
                .field("byte[2]", &self.byte(2usize))
                .field("byte[3]", &self.byte(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dr {{ byte[0]: {=u8:?}, byte[1]: {=u8:?}, byte[2]: {=u8:?}, byte[3]: {=u8:?} }}",
                self.byte(0usize),
                self.byte(1usize),
                self.byte(2usize),
                self.byte(3usize)
            )
        }
    }
    #[doc = "PSSI interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Data buffer overrun/underrun interrupt status clear Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS."]
        #[inline(always)]
        pub const fn ovr_isc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer overrun/underrun interrupt status clear Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS."]
        #[inline(always)]
        pub fn set_ovr_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            f.debug_struct("Icr").field("ovr_isc", &self.ovr_isc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Icr {{ ovr_isc: {=bool:?} }}", self.ovr_isc())
        }
    }
    #[doc = "PSSI interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Data buffer overrun/underrun interrupt enable."]
        #[inline(always)]
        pub const fn ovr_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer overrun/underrun interrupt enable."]
        #[inline(always)]
        pub fn set_ovr_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier").field("ovr_ie", &self.ovr_ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ier {{ ovr_ie: {=bool:?} }}", self.ovr_ie())
        }
    }
    #[doc = "PSSI masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mis(pub u32);
    impl Mis {
        #[doc = "Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1."]
        #[inline(always)]
        pub const fn ovr_mis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1."]
        #[inline(always)]
        pub fn set_ovr_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Mis {
        #[inline(always)]
        fn default() -> Mis {
            Mis(0)
        }
    }
    impl core::fmt::Debug for Mis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mis").field("ovr_mis", &self.ovr_mis()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mis {{ ovr_mis: {=bool:?} }}", self.ovr_mis())
        }
    }
    #[doc = "PSSI raw interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ris(pub u32);
    impl Ris {
        #[doc = "Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR."]
        #[inline(always)]
        pub const fn ovr_ris(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR."]
        #[inline(always)]
        pub fn set_ovr_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ris {
        #[inline(always)]
        fn default() -> Ris {
            Ris(0)
        }
    }
    impl core::fmt::Debug for Ris {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ris").field("ovr_ris", &self.ovr_ris()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ris {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ris {{ ovr_ris: {=bool:?} }}", self.ovr_ris())
        }
    }
    #[doc = "PSSI status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "FIFO is ready to transfer four bytes."]
        #[inline(always)]
        pub const fn rtt4b(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO is ready to transfer four bytes."]
        #[inline(always)]
        pub fn set_rtt4b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO is ready to transfer one byte."]
        #[inline(always)]
        pub const fn rtt1b(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO is ready to transfer one byte."]
        #[inline(always)]
        pub fn set_rtt1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("rtt4b", &self.rtt4b())
                .field("rtt1b", &self.rtt1b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ rtt4b: {=bool:?}, rtt1b: {=bool:?} }}",
                self.rtt4b(),
                self.rtt1b()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckpol {
        #[doc = "Falling edge active for inputs or rising edge active for outputs."]
        FALLING_EDGE = 0x0,
        #[doc = "Rising edge active for inputs or falling edge active for outputs."]
        RISING_EDGE = 0x01,
    }
    impl Ckpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckpol {
        #[inline(always)]
        fn from(val: u8) -> Ckpol {
            Ckpol::from_bits(val)
        }
    }
    impl From<Ckpol> for u8 {
        #[inline(always)]
        fn from(val: Ckpol) -> u8 {
            Ckpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Depol {
        #[doc = "PSSI_DE active low (0 indicates that data is valid)."]
        ACTIVE_LOW = 0x0,
        #[doc = "PSSI_DE active high (1 indicates that data is valid)."]
        ACTIVE_HIGH = 0x01,
    }
    impl Depol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Depol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Depol {
        #[inline(always)]
        fn from(val: u8) -> Depol {
            Depol::from_bits(val)
        }
    }
    impl From<Depol> for u8 {
        #[inline(always)]
        fn from(val: Depol) -> u8 {
            Depol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Derdycfg {
        #[doc = "PSSI_DE and PSSI_RDY both disabled."]
        DISABLED = 0x0,
        #[doc = "Only PSSI_RDY enabled."]
        RDY = 0x01,
        #[doc = "Only PSSI_DE enabled."]
        DE = 0x02,
        #[doc = "Both PSSI_RDY and PSSI_DE alternate functions enabled."]
        RDY_DE_ALT = 0x03,
        #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin."]
        RDY_DE = 0x04,
        #[doc = "Only PSSI_RDY function enabled, but mapped to PSSI_DE pin."]
        RDY_REMAPPED = 0x05,
        #[doc = "Only PSSI_DE function enabled, but mapped to PSSI_RDY pin."]
        DE_REMAPPED = 0x06,
        #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin."]
        RDY_DE_BIDI = 0x07,
    }
    impl Derdycfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Derdycfg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Derdycfg {
        #[inline(always)]
        fn from(val: u8) -> Derdycfg {
            Derdycfg::from_bits(val)
        }
    }
    impl From<Derdycfg> for u8 {
        #[inline(always)]
        fn from(val: Derdycfg) -> u8 {
            Derdycfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Edm {
        #[doc = "Interface captures 8-bit data on every parallel data clock."]
        BIT_WIDTH8 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "The interface captures 16-bit data on every parallel data clock."]
        BIT_WIDTH16 = 0x03,
    }
    impl Edm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Edm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Edm {
        #[inline(always)]
        fn from(val: u8) -> Edm {
            Edm::from_bits(val)
        }
    }
    impl From<Edm> for u8 {
        #[inline(always)]
        fn from(val: Edm) -> u8 {
            Edm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Outen {
        #[doc = "Data is input synchronously with PSSI_PDCK."]
        RECEIVE_MODE = 0x0,
        #[doc = "Data is output synchronously with PSSI_PDCK."]
        TRANSMIT_MODE = 0x01,
    }
    impl Outen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Outen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Outen {
        #[inline(always)]
        fn from(val: u8) -> Outen {
            Outen::from_bits(val)
        }
    }
    impl From<Outen> for u8 {
        #[inline(always)]
        fn from(val: Outen) -> u8 {
            Outen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rdypol {
        #[doc = "PSSI_RDY active low (0 indicates that the receiver is ready to receive)."]
        ACTIVE_LOW = 0x0,
        #[doc = "PSSI_RDY active high (1 indicates that the receiver is ready to receive)."]
        ACTIVE_HIGH = 0x01,
    }
    impl Rdypol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rdypol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rdypol {
        #[inline(always)]
        fn from(val: u8) -> Rdypol {
            Rdypol::from_bits(val)
        }
    }
    impl From<Rdypol> for u8 {
        #[inline(always)]
        fn from(val: Rdypol) -> u8 {
            Rdypol::to_bits(val)
        }
    }
}
