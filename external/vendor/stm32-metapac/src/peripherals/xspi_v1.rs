#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "XSPI register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xspi {
    ptr: *mut u8,
}
unsafe impl Send for Xspi {}
unsafe impl Sync for Xspi {}
impl Xspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "XSPI control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "XSPI device configuration register 1"]
    #[inline(always)]
    pub const fn dcr1(self) -> crate::common::Reg<regs::Dcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "XSPI device configuration register 2"]
    #[inline(always)]
    pub const fn dcr2(self) -> crate::common::Reg<regs::Dcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "XSPI device configuration register 3"]
    #[inline(always)]
    pub const fn dcr3(self) -> crate::common::Reg<regs::Dcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "XSPI device configuration register 4"]
    #[inline(always)]
    pub const fn dcr4(self) -> crate::common::Reg<regs::Dcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "XSPI status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "XSPI flag clear register"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "XSPI data length register"]
    #[inline(always)]
    pub const fn dlr(self) -> crate::common::Reg<regs::Dlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "XSPIaddress register"]
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::Ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "XSPI data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "XSPI polling status mask register"]
    #[inline(always)]
    pub const fn psmkr(self) -> crate::common::Reg<regs::Psmkr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "XSPI polling status match register"]
    #[inline(always)]
    pub const fn psmar(self) -> crate::common::Reg<regs::Psmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "XSPI polling interval register"]
    #[inline(always)]
    pub const fn pir(self) -> crate::common::Reg<regs::Pir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "XSPI communication configuration register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "XSPI timing configuration register"]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "XSPI instruction register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "XSPI alternate bytes register"]
    #[inline(always)]
    pub const fn abr(self) -> crate::common::Reg<regs::Abr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "XSPI low-power timeout register"]
    #[inline(always)]
    pub const fn lptr(self) -> crate::common::Reg<regs::Lptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "XSPI wrap communication configuration register"]
    #[inline(always)]
    pub const fn wpccr(self) -> crate::common::Reg<regs::Wpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "XSPI wrap timing configuration register"]
    #[inline(always)]
    pub const fn wptcr(self) -> crate::common::Reg<regs::Wptcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "XSPI wrap instruction register"]
    #[inline(always)]
    pub const fn wpir(self) -> crate::common::Reg<regs::Wpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "XSPI wrap alternate byte register"]
    #[inline(always)]
    pub const fn wpabr(self) -> crate::common::Reg<regs::Wpabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "XSPI write communication configuration register"]
    #[inline(always)]
    pub const fn wccr(self) -> crate::common::Reg<regs::Wccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "XSPI write timing configuration register"]
    #[inline(always)]
    pub const fn wtcr(self) -> crate::common::Reg<regs::Wtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "XSPI write instruction register"]
    #[inline(always)]
    pub const fn wir(self) -> crate::common::Reg<regs::Wir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "XSPI write alternate byte register"]
    #[inline(always)]
    pub const fn wabr(self) -> crate::common::Reg<regs::Wabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "XSPI HyperBus latency configuration register"]
    #[inline(always)]
    pub const fn hlcr(self) -> crate::common::Reg<regs::Hlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "XSPI full-cycle calibration configuration"]
    #[inline(always)]
    pub const fn calfcr(self) -> crate::common::Reg<regs::Calfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "XSPI DLL master calibration configuration"]
    #[inline(always)]
    pub const fn calmr(self) -> crate::common::Reg<regs::Calmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "XSPI DLL slave output calibration configuration"]
    #[inline(always)]
    pub const fn calsor(self) -> crate::common::Reg<regs::Calsor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "XSPI DLL slave input calibration configuration"]
    #[inline(always)]
    pub const fn calsir(self) -> crate::common::Reg<regs::Calsir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
}
pub mod regs {
    #[doc = "XSPI alternate bytes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Abr(pub u32);
    impl Abr {
        #[doc = "Alternate bytes Optional data to be sent to the external SPI device right after the address."]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alternate bytes Optional data to be sent to the external SPI device right after the address."]
        #[inline(always)]
        pub fn set_alternate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Abr {
        #[inline(always)]
        fn default() -> Abr {
            Abr(0)
        }
    }
    impl core::fmt::Debug for Abr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Abr").field("alternate", &self.alternate()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Abr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Abr {{ alternate: {=u32:?} }}", self.alternate())
        }
    }
    #[doc = "XSPIaddress register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ar(pub u32);
    impl Ar {
        #[doc = "Address Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 0. Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (memory-mapped mode). Some memory specifications consider that each address corresponds to a 16-bit value. XSPI considers that each address corresponds to an 8-bit value. So the software needs to multiple the address by two when accessing the memory registers."]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 0. Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (memory-mapped mode). Some memory specifications consider that each address corresponds to a 16-bit value. XSPI considers that each address corresponds to an 8-bit value. So the software needs to multiple the address by two when accessing the memory registers."]
        #[inline(always)]
        pub fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ar {
        #[inline(always)]
        fn default() -> Ar {
            Ar(0)
        }
    }
    impl core::fmt::Debug for Ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ar").field("address", &self.address()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ar {{ address: {=u32:?} }}", self.address())
        }
    }
    #[doc = "XSPI full-cycle calibration configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfcr(pub u32);
    impl Calfcr {
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_coarse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Max value This bit gets set when the memory-clock period is outside the range of DLL master, in which case XSPI_CALFCR and XSPI_CALSR are updated with the values for the maximum delay."]
        #[inline(always)]
        pub const fn calmax(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Max value This bit gets set when the memory-clock period is outside the range of DLL master, in which case XSPI_CALFCR and XSPI_CALSR are updated with the values for the maximum delay."]
        #[inline(always)]
        pub fn set_calmax(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Calfcr {
        #[inline(always)]
        fn default() -> Calfcr {
            Calfcr(0)
        }
    }
    impl core::fmt::Debug for Calfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calfcr")
                .field("fine", &self.fine())
                .field("coarse", &self.coarse())
                .field("calmax", &self.calmax())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calfcr {{ fine: {=u8:?}, coarse: {=u8:?}, calmax: {=bool:?} }}",
                self.fine(),
                self.coarse(),
                self.calmax()
            )
        }
    }
    #[doc = "XSPI DLL master calibration configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calmr(pub u32);
    impl Calmr {
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_coarse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Calmr {
        #[inline(always)]
        fn default() -> Calmr {
            Calmr(0)
        }
    }
    impl core::fmt::Debug for Calmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calmr")
                .field("fine", &self.fine())
                .field("coarse", &self.coarse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calmr {{ fine: {=u8:?}, coarse: {=u8:?} }}",
                self.fine(),
                self.coarse()
            )
        }
    }
    #[doc = "XSPI DLL slave input calibration configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calsir(pub u32);
    impl Calsir {
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_coarse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Calsir {
        #[inline(always)]
        fn default() -> Calsir {
            Calsir(0)
        }
    }
    impl core::fmt::Debug for Calsir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calsir")
                .field("fine", &self.fine())
                .field("coarse", &self.coarse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calsir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calsir {{ fine: {=u8:?}, coarse: {=u8:?} }}",
                self.fine(),
                self.coarse()
            )
        }
    }
    #[doc = "XSPI DLL slave output calibration configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calsor(pub u32);
    impl Calsor {
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_coarse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Calsor {
        #[inline(always)]
        fn default() -> Calsor {
            Calsor(0)
        }
    }
    impl core::fmt::Debug for Calsor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calsor")
                .field("fine", &self.fine())
                .field("coarse", &self.coarse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calsor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calsor {{ fine: {=u8:?}, coarse: {=u8:?} }}",
                self.fine(),
                self.coarse()
            )
        }
    }
    #[doc = "XSPI communication configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn imode(&self) -> super::vals::CcrImode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::CcrImode::from_bits(val as u8)
        }
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_imode(&mut self, val: super::vals::CcrImode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Instruction size This bit defines instruction size."]
        #[inline(always)]
        pub const fn isize(&self) -> super::vals::CcrIsize {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::CcrIsize::from_bits(val as u8)
        }
        #[doc = "Instruction size This bit defines instruction size."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: super::vals::CcrIsize) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode This field defines the address phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn admode(&self) -> super::vals::CcrAdmode {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::CcrAdmode::from_bits(val as u8)
        }
        #[doc = "Address mode This field defines the address phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_admode(&mut self, val: super::vals::CcrAdmode) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Address double transfer rate This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address double transfer rate This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub const fn adsize(&self) -> super::vals::CcrAdsize {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::CcrAdsize::from_bits(val as u8)
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: super::vals::CcrAdsize) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn abmode(&self) -> super::vals::CcrAbmode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::CcrAbmode::from_bits(val as u8)
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: super::vals::CcrAbmode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase. Note: This field can be written only when BUSY = 0."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase. Note: This field can be written only when BUSY = 0."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub const fn absize(&self) -> super::vals::CcrAbsize {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::CcrAbsize::from_bits(val as u8)
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: super::vals::CcrAbsize) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode This field defines the data phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn dmode(&self) -> super::vals::CcrDmode {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::CcrDmode::from_bits(val as u8)
        }
        #[doc = "Data mode This field defines the data phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: super::vals::CcrDmode) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Data double transfer rate This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Data double transfer rate This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DQS enable This bit enables the data strobe management."]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DQS enable This bit enables the data strobe management."]
        #[inline(always)]
        pub fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("imode", &self.imode())
                .field("idtr", &self.idtr())
                .field("isize", &self.isize())
                .field("admode", &self.admode())
                .field("addtr", &self.addtr())
                .field("adsize", &self.adsize())
                .field("abmode", &self.abmode())
                .field("abdtr", &self.abdtr())
                .field("absize", &self.absize())
                .field("dmode", &self.dmode())
                .field("ddtr", &self.ddtr())
                .field("dqse", &self.dqse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccr {{ imode: {:?}, idtr: {=bool:?}, isize: {:?}, admode: {:?}, addtr: {=bool:?}, adsize: {:?}, abmode: {:?}, abdtr: {=bool:?}, absize: {:?}, dmode: {:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse ())
        }
    }
    #[doc = "XSPI control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable This bit enables the XSPI. The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. Note: In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable This bit enables the XSPI. The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. Note: In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
        #[inline(always)]
        pub const fn abort(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
        #[inline(always)]
        pub fn set_abort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA enable In indirect mode, the DMA can be used to input or output data via XSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable In indirect mode, the DMA can be used to input or output data via XSPI_DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timeout counter enable This bit is valid only when the memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub const fn tcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout counter enable This bit is valid only when the memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub fn set_tcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Dual-memory configuration This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub const fn dmm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Dual-memory configuration This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub fn set_dmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIFO threshold level This field defines, in indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in XSPI_SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[5:0\\]
value."]
        #[inline(always)]
        pub const fn fthres(&self) -> super::vals::Fthres {
            let val = (self.0 >> 8usize) & 0x3f;
            super::vals::Fthres::from_bits(val as u8)
        }
        #[doc = "FIFO threshold level This field defines, in indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in XSPI_SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[5:0\\]
value."]
        #[inline(always)]
        pub fn set_fthres(&mut self, val: super::vals::Fthres) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
        }
        #[doc = "Transfer error interrupt enable This bit enables the transfer error interrupt."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable This bit enables the transfer error interrupt."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
        #[inline(always)]
        pub const fn ftie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
        #[inline(always)]
        pub fn set_ftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Status match interrupt enable This bit enables the status match interrupt."]
        #[inline(always)]
        pub const fn smie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Status match interrupt enable This bit enables the status match interrupt."]
        #[inline(always)]
        pub fn set_smie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timeout interrupt enable This bit enables the timeout interrupt."]
        #[inline(always)]
        pub const fn toie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout interrupt enable This bit enables the timeout interrupt."]
        #[inline(always)]
        pub fn set_toie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Automatic status-polling mode stop This bit determines if the automatic status-polling is stopped after a match. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub const fn apms(&self) -> super::vals::Apms {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Apms::from_bits(val as u8)
        }
        #[doc = "Automatic status-polling mode stop This bit determines if the automatic status-polling is stopped after a match. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub fn set_apms(&mut self, val: super::vals::Apms) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Polling match mode This bit indicates which method must be used to determine a match during the automatic status-polling mode. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub const fn pmm(&self) -> super::vals::Pmm {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Pmm::from_bits(val as u8)
        }
        #[doc = "Polling match mode This bit indicates which method must be used to determine a match during the automatic status-polling mode. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub fn set_pmm(&mut self, val: super::vals::Pmm) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "chip select selection This bit indicates if the XSPI must activate NCS1 or NCS2. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub const fn cssel(&self) -> super::vals::Cssel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Cssel::from_bits(val as u8)
        }
        #[doc = "chip select selection This bit indicates if the XSPI must activate NCS1 or NCS2. Note: This bit can be modified only when BUSY = 0."]
        #[inline(always)]
        pub fn set_cssel(&mut self, val: super::vals::Cssel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Functional mode This field defines the XSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state. Note: This bitfield can be modified only when BUSY = 0."]
        #[inline(always)]
        pub const fn fmode(&self) -> super::vals::Fmode {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Fmode::from_bits(val as u8)
        }
        #[doc = "Functional mode This field defines the XSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state. Note: This bitfield can be modified only when BUSY = 0."]
        #[inline(always)]
        pub fn set_fmode(&mut self, val: super::vals::Fmode) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Flash select"]
        #[inline(always)]
        pub const fn msel(&self) -> super::vals::Msel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Msel::from_bits(val as u8)
        }
        #[doc = "Flash select"]
        #[inline(always)]
        pub fn set_msel(&mut self, val: super::vals::Msel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
                .field("en", &self.en())
                .field("abort", &self.abort())
                .field("dmaen", &self.dmaen())
                .field("tcen", &self.tcen())
                .field("dmm", &self.dmm())
                .field("fthres", &self.fthres())
                .field("teie", &self.teie())
                .field("tcie", &self.tcie())
                .field("ftie", &self.ftie())
                .field("smie", &self.smie())
                .field("toie", &self.toie())
                .field("apms", &self.apms())
                .field("pmm", &self.pmm())
                .field("cssel", &self.cssel())
                .field("fmode", &self.fmode())
                .field("msel", &self.msel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, abort: {=bool:?}, dmaen: {=bool:?}, tcen: {=bool:?}, dmm: {=bool:?}, fthres: {:?}, teie: {=bool:?}, tcie: {=bool:?}, ftie: {=bool:?}, smie: {=bool:?}, toie: {=bool:?}, apms: {:?}, pmm: {:?}, cssel: {:?}, fmode: {:?}, msel: {:?} }}" , self . en () , self . abort () , self . dmaen () , self . tcen () , self . dmm () , self . fthres () , self . teie () , self . tcie () , self . ftie () , self . smie () , self . toie () , self . apms () , self . pmm () , self . cssel () , self . fmode () , self . msel ())
        }
    }
    #[doc = "XSPI device configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr1(pub u32);
    impl Dcr1 {
        #[doc = "clock mode 0/mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
        #[inline(always)]
        pub const fn ckmode(&self) -> super::vals::Ckmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ckmode::from_bits(val as u8)
        }
        #[doc = "clock mode 0/mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: super::vals::Ckmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Free running clock This bit configures the free running clock."]
        #[inline(always)]
        pub const fn frck(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Free running clock This bit configures the free running clock."]
        #[inline(always)]
        pub fn set_frck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
        #[inline(always)]
        pub const fn csht(&self) -> super::vals::Csht {
            let val = (self.0 >> 8usize) & 0x3f;
            super::vals::Csht::from_bits(val as u8)
        }
        #[doc = "Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
        #[inline(always)]
        pub fn set_csht(&mut self, val: super::vals::Csht) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
        }
        #[doc = "Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2<sup>\\[DEVSIZE+1\\]</sup>. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256 Mbytes. In regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
        #[inline(always)]
        pub const fn devsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2<sup>\\[DEVSIZE+1\\]</sup>. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256 Mbytes. In regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
        #[inline(always)]
        pub fn set_devsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Dcr1 {
        #[inline(always)]
        fn default() -> Dcr1 {
            Dcr1(0)
        }
    }
    impl core::fmt::Debug for Dcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr1")
                .field("ckmode", &self.ckmode())
                .field("frck", &self.frck())
                .field("csht", &self.csht())
                .field("devsize", &self.devsize())
                .field("mtyp", &self.mtyp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dcr1 {{ ckmode: {:?}, frck: {=bool:?}, csht: {:?}, devsize: {=u8:?}, mtyp: {:?} }}",
                self.ckmode(),
                self.frck(),
                self.csht(),
                self.devsize(),
                self.mtyp()
            )
        }
    }
    #[doc = "XSPI device configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr2(pub u32);
    impl Dcr2 {
        #[doc = "Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (value + 1). ... For odd clock division factors, the CLK duty cycle is not 50 %. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case XSPI_CALOSR or XSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution."]
        #[inline(always)]
        pub const fn prescaler(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (value + 1). ... For odd clock division factors, the CLK duty cycle is not 50 %. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case XSPI_CALOSR or XSPI_CALISR have been written in the meantime. BUSY stays high during the whole calibration execution."]
        #[inline(always)]
        pub fn set_prescaler(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in XSPI_WPIR. Others: reserved"]
        #[inline(always)]
        pub const fn wrapsize(&self) -> super::vals::Wrapsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Wrapsize::from_bits(val as u8)
        }
        #[doc = "Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in XSPI_WPIR. Others: reserved"]
        #[inline(always)]
        pub fn set_wrapsize(&mut self, val: super::vals::Wrapsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Dcr2 {
        #[inline(always)]
        fn default() -> Dcr2 {
            Dcr2(0)
        }
    }
    impl core::fmt::Debug for Dcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr2")
                .field("prescaler", &self.prescaler())
                .field("wrapsize", &self.wrapsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dcr2 {{ prescaler: {=u8:?}, wrapsize: {:?} }}",
                self.prescaler(),
                self.wrapsize()
            )
        }
    }
    #[doc = "XSPI device configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr3(pub u32);
    impl Dcr3 {
        #[doc = "Maximum transfer This field enables the communication regulation feature. The NCS is released every MAXTRAN+1 clock cycles when the other XSPI request the access to the bus. Others: maximum communication is set to MAXTRAN + 1 bytes."]
        #[inline(always)]
        pub const fn maxtran(&self) -> super::vals::Maxtran {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Maxtran::from_bits(val as u8)
        }
        #[doc = "Maximum transfer This field enables the communication regulation feature. The NCS is released every MAXTRAN+1 clock cycles when the other XSPI request the access to the bus. Others: maximum communication is set to MAXTRAN + 1 bytes."]
        #[inline(always)]
        pub fn set_maxtran(&mut self, val: super::vals::Maxtran) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2<sup>CSBOUND</sup> bytes. Others: NCS boundary set to 2<sup>CSBOUND</sup> bytes"]
        #[inline(always)]
        pub const fn csbound(&self) -> super::vals::Csbound {
            let val = (self.0 >> 16usize) & 0x1f;
            super::vals::Csbound::from_bits(val as u8)
        }
        #[doc = "NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2<sup>CSBOUND</sup> bytes. Others: NCS boundary set to 2<sup>CSBOUND</sup> bytes"]
        #[inline(always)]
        pub fn set_csbound(&mut self, val: super::vals::Csbound) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Dcr3 {
        #[inline(always)]
        fn default() -> Dcr3 {
            Dcr3(0)
        }
    }
    impl core::fmt::Debug for Dcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr3")
                .field("maxtran", &self.maxtran())
                .field("csbound", &self.csbound())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dcr3 {{ maxtran: {:?}, csbound: {:?} }}",
                self.maxtran(),
                self.csbound()
            )
        }
    }
    #[doc = "XSPI device configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr4(pub u32);
    impl Dcr4 {
        #[doc = "Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single-, dual- or quad-SPI mode, because the byte transmission must be completed. Others: maximum communication length is set to REFRESH + 1 clock cycles."]
        #[inline(always)]
        pub const fn refresh(&self) -> super::vals::Refresh {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Refresh::from_bits(val as u32)
        }
        #[doc = "Refresh rate This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single-, dual- or quad-SPI mode, because the byte transmission must be completed. Others: maximum communication length is set to REFRESH + 1 clock cycles."]
        #[inline(always)]
        pub fn set_refresh(&mut self, val: super::vals::Refresh) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dcr4 {
        #[inline(always)]
        fn default() -> Dcr4 {
            Dcr4(0)
        }
    }
    impl core::fmt::Debug for Dcr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr4").field("refresh", &self.refresh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcr4 {{ refresh: {:?} }}", self.refresh())
        }
    }
    #[doc = "XSPI data length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlr(pub u32);
    impl Dlr {
        #[doc = "Data length"]
        #[inline(always)]
        pub const fn dl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data length"]
        #[inline(always)]
        pub fn set_dl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlr {
        #[inline(always)]
        fn default() -> Dlr {
            Dlr(0)
        }
    }
    impl core::fmt::Debug for Dlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlr").field("dl", &self.dl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dlr {{ dl: {=u32:?} }}", self.dl())
        }
    }
    #[doc = "XSPI data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data Data to be sent/received to/from the external SPI device In indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY = 1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In automatic status-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2 bytes, and a word read 4 bytes. Accesses in indirect mode must be aligned to the bottom of this register: A byte read must read DATA\\[7:0\\]
and a half-word read must read DATA\\[15:0\\]."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Data to be sent/received to/from the external SPI device In indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY = 1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In automatic status-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2 bytes, and a word read 4 bytes. Accesses in indirect mode must be aligned to the bottom of this register: A byte read must read DATA\\[7:0\\]
and a half-word read must read DATA\\[15:0\\]."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Dr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "XSPI flag clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Clear transfer error flag Writing 1 clears the TEF flag in the XSPI_SR register."]
        #[inline(always)]
        pub const fn ctef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer error flag Writing 1 clears the TEF flag in the XSPI_SR register."]
        #[inline(always)]
        pub fn set_ctef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear transfer complete flag Writing 1 clears the TCF flag in the XSPI_SR register."]
        #[inline(always)]
        pub const fn ctcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer complete flag Writing 1 clears the TCF flag in the XSPI_SR register."]
        #[inline(always)]
        pub fn set_ctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear status match flag Writing 1 clears the SMF flag in the XSPI_SR register."]
        #[inline(always)]
        pub const fn csmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear status match flag Writing 1 clears the SMF flag in the XSPI_SR register."]
        #[inline(always)]
        pub fn set_csmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear timeout flag Writing 1 clears the TOF flag in the XSPI_SR register."]
        #[inline(always)]
        pub const fn ctof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear timeout flag Writing 1 clears the TOF flag in the XSPI_SR register."]
        #[inline(always)]
        pub fn set_ctof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("ctef", &self.ctef())
                .field("ctcf", &self.ctcf())
                .field("csmf", &self.csmf())
                .field("ctof", &self.ctof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ ctef: {=bool:?}, ctcf: {=bool:?}, csmf: {=bool:?}, ctof: {=bool:?} }}",
                self.ctef(),
                self.ctcf(),
                self.csmf(),
                self.ctof()
            )
        }
    }
    #[doc = "XSPI HyperBus latency configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hlcr(pub u32);
    impl Hlcr {
        #[doc = "Latency mode This bit selects the Latency mode. Note: This bit must be set when using the dual-octal HyperBus configuration."]
        #[inline(always)]
        pub const fn lm(&self) -> super::vals::Lm {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lm::from_bits(val as u8)
        }
        #[doc = "Latency mode This bit selects the Latency mode. Note: This bit must be set when using the dual-octal HyperBus configuration."]
        #[inline(always)]
        pub fn set_lm(&mut self, val: super::vals::Lm) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Write zero latency This bit enables zero latency on write operations."]
        #[inline(always)]
        pub const fn wzl(&self) -> super::vals::Wzl {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wzl::from_bits(val as u8)
        }
        #[doc = "Write zero latency This bit enables zero latency on write operations."]
        #[inline(always)]
        pub fn set_wzl(&mut self, val: super::vals::Wzl) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Access time Device access time expressed in number of communication clock cycles"]
        #[inline(always)]
        pub const fn tacc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Access time Device access time expressed in number of communication clock cycles"]
        #[inline(always)]
        pub fn set_tacc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Read write recovery time Device read write recovery time expressed in number of communication clock cycles"]
        #[inline(always)]
        pub const fn trwr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Read write recovery time Device read write recovery time expressed in number of communication clock cycles"]
        #[inline(always)]
        pub fn set_trwr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Hlcr {
        #[inline(always)]
        fn default() -> Hlcr {
            Hlcr(0)
        }
    }
    impl core::fmt::Debug for Hlcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hlcr")
                .field("lm", &self.lm())
                .field("wzl", &self.wzl())
                .field("tacc", &self.tacc())
                .field("trwr", &self.trwr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hlcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hlcr {{ lm: {:?}, wzl: {:?}, tacc: {=u8:?}, trwr: {=u8:?} }}",
                self.lm(),
                self.wzl(),
                self.tacc(),
                self.trwr()
            )
        }
    }
    #[doc = "XSPI instruction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub fn set_instruction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Ir").field("instruction", &self.instruction()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ir {{ instruction: {=u32:?} }}", self.instruction())
        }
    }
    #[doc = "XSPI low-power timeout register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lptr(pub u32);
    impl Lptr {
        #[doc = "Timeout period After each access in memory-mapped mode, the XSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the XSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
        #[inline(always)]
        pub const fn timeout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout period After each access in memory-mapped mode, the XSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the XSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lptr {
        #[inline(always)]
        fn default() -> Lptr {
            Lptr(0)
        }
    }
    impl core::fmt::Debug for Lptr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lptr").field("timeout", &self.timeout()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lptr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lptr {{ timeout: {=u16:?} }}", self.timeout())
        }
    }
    #[doc = "XSPI polling interval register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pir(pub u32);
    impl Pir {
        #[doc = "Polling interval Number of CLK cycle between a read during the automatic status-polling phases"]
        #[inline(always)]
        pub const fn interval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Polling interval Number of CLK cycle between a read during the automatic status-polling phases"]
        #[inline(always)]
        pub fn set_interval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pir {
        #[inline(always)]
        fn default() -> Pir {
            Pir(0)
        }
    }
    impl core::fmt::Debug for Pir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pir").field("interval", &self.interval()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pir {{ interval: {=u16:?} }}", self.interval())
        }
    }
    #[doc = "XSPI polling status match register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psmar(pub u32);
    impl Psmar {
        #[doc = "Status match Value to be compared with the masked status register to get a match"]
        #[inline(always)]
        pub const fn match_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Status match Value to be compared with the masked status register to get a match"]
        #[inline(always)]
        pub fn set_match_(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Psmar {
        #[inline(always)]
        fn default() -> Psmar {
            Psmar(0)
        }
    }
    impl core::fmt::Debug for Psmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psmar").field("match_", &self.match_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Psmar {{ match_: {=u32:?} }}", self.match_())
        }
    }
    #[doc = "XSPI polling status mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psmkr(pub u32);
    impl Psmkr {
        #[doc = "Status mask Mask to be applied to the status bytes received in automatic status-polling mode For bit n:"]
        #[inline(always)]
        pub const fn mask(&self) -> super::vals::Mask {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Mask::from_bits(val as u32)
        }
        #[doc = "Status mask Mask to be applied to the status bytes received in automatic status-polling mode For bit n:"]
        #[inline(always)]
        pub fn set_mask(&mut self, val: super::vals::Mask) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Psmkr {
        #[inline(always)]
        fn default() -> Psmkr {
            Psmkr(0)
        }
    }
    impl core::fmt::Debug for Psmkr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psmkr").field("mask", &self.mask()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psmkr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Psmkr {{ mask: {:?} }}", self.mask())
        }
    }
    #[doc = "XSPI status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Transfer error flag This bit is set in indirect mode when an invalid address is being accessed in indirect mode. It is cleared by writing 1 to CTEF."]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error flag This bit is set in indirect mode when an invalid address is being accessed in indirect mode. It is cleared by writing 1 to CTEF."]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete flag This bit is set in indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete flag This bit is set in indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO threshold flag In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In automatic status-polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO threshold flag In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In automatic status-polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
        #[inline(always)]
        pub fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Status match flag This bit is set in automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (XSPI_PSMAR). It is cleared by writing 1 to CSMF."]
        #[inline(always)]
        pub const fn smf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Status match flag This bit is set in automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (XSPI_PSMAR). It is cleared by writing 1 to CSMF."]
        #[inline(always)]
        pub fn set_smf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF."]
        #[inline(always)]
        pub const fn tof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF."]
        #[inline(always)]
        pub fn set_tof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Busy This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Busy This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 64 when it is full. In automatic-status polling mode, FLEVEL is zero."]
        #[inline(always)]
        pub const fn flevel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 64 when it is full. In automatic-status polling mode, FLEVEL is zero."]
        #[inline(always)]
        pub fn set_flevel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
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
                .field("tef", &self.tef())
                .field("tcf", &self.tcf())
                .field("ftf", &self.ftf())
                .field("smf", &self.smf())
                .field("tof", &self.tof())
                .field("busy", &self.busy())
                .field("flevel", &self.flevel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ tef: {=bool:?}, tcf: {=bool:?}, ftf: {=bool:?}, smf: {=bool:?}, tof: {=bool:?}, busy: {=bool:?}, flevel: {=u8:?} }}" , self . tef () , self . tcf () , self . ftf () , self . smf () , self . tof () , self . busy () , self . flevel ())
        }
    }
    #[doc = "XSPI timing configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcr(pub u32);
    impl Tcr {
        #[doc = "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31)."]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31)."]
        #[inline(always)]
        pub fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay hold quarter cycle"]
        #[inline(always)]
        pub const fn dhqc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Delay hold quarter cycle"]
        #[inline(always)]
        pub fn set_dhqc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Sample shift By default, the XSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
        #[inline(always)]
        pub const fn sshift(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Sample shift By default, the XSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
        #[inline(always)]
        pub fn set_sshift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Tcr {
        #[inline(always)]
        fn default() -> Tcr {
            Tcr(0)
        }
    }
    impl core::fmt::Debug for Tcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tcr")
                .field("dcyc", &self.dcyc())
                .field("dhqc", &self.dhqc())
                .field("sshift", &self.sshift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tcr {{ dcyc: {=u8:?}, dhqc: {=bool:?}, sshift: {=bool:?} }}",
                self.dcyc(),
                self.dhqc(),
                self.sshift()
            )
        }
    }
    #[doc = "XSPI write alternate byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wabr(pub u32);
    impl Wabr {
        #[doc = "Alternate bytes Optional data to be sent to the external SPI device right after the address"]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alternate bytes Optional data to be sent to the external SPI device right after the address"]
        #[inline(always)]
        pub fn set_alternate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wabr {
        #[inline(always)]
        fn default() -> Wabr {
            Wabr(0)
        }
    }
    impl core::fmt::Debug for Wabr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wabr").field("alternate", &self.alternate()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wabr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wabr {{ alternate: {=u32:?} }}", self.alternate())
        }
    }
    #[doc = "XSPI write communication configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wccr(pub u32);
    impl Wccr {
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn imode(&self) -> super::vals::WccrImode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::WccrImode::from_bits(val as u8)
        }
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_imode(&mut self, val: super::vals::WccrImode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Instruction size This bit defines instruction size:"]
        #[inline(always)]
        pub const fn isize(&self) -> super::vals::WccrIsize {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::WccrIsize::from_bits(val as u8)
        }
        #[doc = "Instruction size This bit defines instruction size:"]
        #[inline(always)]
        pub fn set_isize(&mut self, val: super::vals::WccrIsize) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode This field defines the address phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn admode(&self) -> super::vals::WccrAdmode {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::WccrAdmode::from_bits(val as u8)
        }
        #[doc = "Address mode This field defines the address phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_admode(&mut self, val: super::vals::WccrAdmode) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Address double transfer rate This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address double transfer rate This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub const fn adsize(&self) -> super::vals::WccrAdsize {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::WccrAdsize::from_bits(val as u8)
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: super::vals::WccrAdsize) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode This field defines the alternate-byte phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn abmode(&self) -> super::vals::WccrAbmode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::WccrAbmode::from_bits(val as u8)
        }
        #[doc = "Alternate-byte mode This field defines the alternate-byte phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: super::vals::WccrAbmode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size This field defines alternate bytes size:"]
        #[inline(always)]
        pub const fn absize(&self) -> super::vals::WccrAbsize {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::WccrAbsize::from_bits(val as u8)
        }
        #[doc = "Alternate bytes size This field defines alternate bytes size:"]
        #[inline(always)]
        pub fn set_absize(&mut self, val: super::vals::WccrAbsize) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode This field defines the data phase mode of operation."]
        #[inline(always)]
        pub const fn dmode(&self) -> super::vals::WccrDmode {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::WccrDmode::from_bits(val as u8)
        }
        #[doc = "Data mode This field defines the data phase mode of operation."]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: super::vals::WccrDmode) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "data double transfer rate This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "data double transfer rate This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DQS enable This bit enables the data strobe management."]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DQS enable This bit enables the data strobe management."]
        #[inline(always)]
        pub fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Wccr {
        #[inline(always)]
        fn default() -> Wccr {
            Wccr(0)
        }
    }
    impl core::fmt::Debug for Wccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wccr")
                .field("imode", &self.imode())
                .field("idtr", &self.idtr())
                .field("isize", &self.isize())
                .field("admode", &self.admode())
                .field("addtr", &self.addtr())
                .field("adsize", &self.adsize())
                .field("abmode", &self.abmode())
                .field("abdtr", &self.abdtr())
                .field("absize", &self.absize())
                .field("dmode", &self.dmode())
                .field("ddtr", &self.ddtr())
                .field("dqse", &self.dqse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wccr {{ imode: {:?}, idtr: {=bool:?}, isize: {:?}, admode: {:?}, addtr: {=bool:?}, adsize: {:?}, abmode: {:?}, abdtr: {=bool:?}, absize: {:?}, dmode: {:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse ())
        }
    }
    #[doc = "XSPI write instruction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wir(pub u32);
    impl Wir {
        #[doc = "Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub fn set_instruction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wir {
        #[inline(always)]
        fn default() -> Wir {
            Wir(0)
        }
    }
    impl core::fmt::Debug for Wir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wir").field("instruction", &self.instruction()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wir {{ instruction: {=u32:?} }}", self.instruction())
        }
    }
    #[doc = "XSPI wrap alternate byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpabr(pub u32);
    impl Wpabr {
        #[doc = "Alternate bytes Optional data to be sent to the external SPI device right after the address"]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alternate bytes Optional data to be sent to the external SPI device right after the address"]
        #[inline(always)]
        pub fn set_alternate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wpabr {
        #[inline(always)]
        fn default() -> Wpabr {
            Wpabr(0)
        }
    }
    impl core::fmt::Debug for Wpabr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpabr").field("alternate", &self.alternate()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpabr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wpabr {{ alternate: {=u32:?} }}", self.alternate())
        }
    }
    #[doc = "XSPI wrap communication configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpccr(pub u32);
    impl Wpccr {
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn imode(&self) -> super::vals::WpccrImode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::WpccrImode::from_bits(val as u8)
        }
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_imode(&mut self, val: super::vals::WpccrImode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction double transfer rate This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Instruction size This field defines instruction size."]
        #[inline(always)]
        pub const fn isize(&self) -> super::vals::WpccrIsize {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::WpccrIsize::from_bits(val as u8)
        }
        #[doc = "Instruction size This field defines instruction size."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: super::vals::WpccrIsize) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode This field defines the address phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub const fn admode(&self) -> super::vals::WpccrAdmode {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::WpccrAdmode::from_bits(val as u8)
        }
        #[doc = "Address mode This field defines the address phase mode of operation. Others: reserved"]
        #[inline(always)]
        pub fn set_admode(&mut self, val: super::vals::WpccrAdmode) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Address double transfer rate This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address double transfer rate This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub const fn adsize(&self) -> super::vals::WpccrAdsize {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::WpccrAdsize::from_bits(val as u8)
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: super::vals::WpccrAdsize) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation."]
        #[inline(always)]
        pub const fn abmode(&self) -> super::vals::WpccrAbmode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::WpccrAbmode::from_bits(val as u8)
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation."]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: super::vals::WpccrAbmode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub const fn absize(&self) -> super::vals::WpccrAbsize {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::WpccrAbsize::from_bits(val as u8)
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: super::vals::WpccrAbsize) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode This field defines the data phase mode of operation. 101; data on 16 lines Others: reserved"]
        #[inline(always)]
        pub const fn dmode(&self) -> super::vals::WpccrDmode {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::WpccrDmode::from_bits(val as u8)
        }
        #[doc = "Data mode This field defines the data phase mode of operation. 101; data on 16 lines Others: reserved"]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: super::vals::WpccrDmode) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Data double transfer rate This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Data double transfer rate This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DQS enable This bit enables the data strobe management."]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DQS enable This bit enables the data strobe management."]
        #[inline(always)]
        pub fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Wpccr {
        #[inline(always)]
        fn default() -> Wpccr {
            Wpccr(0)
        }
    }
    impl core::fmt::Debug for Wpccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpccr")
                .field("imode", &self.imode())
                .field("idtr", &self.idtr())
                .field("isize", &self.isize())
                .field("admode", &self.admode())
                .field("addtr", &self.addtr())
                .field("adsize", &self.adsize())
                .field("abmode", &self.abmode())
                .field("abdtr", &self.abdtr())
                .field("absize", &self.absize())
                .field("dmode", &self.dmode())
                .field("ddtr", &self.ddtr())
                .field("dqse", &self.dqse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wpccr {{ imode: {:?}, idtr: {=bool:?}, isize: {:?}, admode: {:?}, addtr: {=bool:?}, adsize: {:?}, abmode: {:?}, abdtr: {=bool:?}, absize: {:?}, dmode: {:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse ())
        }
    }
    #[doc = "XSPI wrap instruction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpir(pub u32);
    impl Wpir {
        #[doc = "Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub fn set_instruction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wpir {
        #[inline(always)]
        fn default() -> Wpir {
            Wpir(0)
        }
    }
    impl core::fmt::Debug for Wpir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpir")
                .field("instruction", &self.instruction())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wpir {{ instruction: {=u32:?} }}", self.instruction())
        }
    }
    #[doc = "XSPI wrap timing configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wptcr(pub u32);
    impl Wptcr {
        #[doc = "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31)."]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31)."]
        #[inline(always)]
        pub fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay hold quarter cycle Add a quarter cycle delay on the outputs in DTR communication to match hold requirement."]
        #[inline(always)]
        pub const fn dhqc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Delay hold quarter cycle Add a quarter cycle delay on the outputs in DTR communication to match hold requirement."]
        #[inline(always)]
        pub fn set_dhqc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Sample shift By default, the XSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR = 1)."]
        #[inline(always)]
        pub const fn sshift(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Sample shift By default, the XSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR = 1)."]
        #[inline(always)]
        pub fn set_sshift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Wptcr {
        #[inline(always)]
        fn default() -> Wptcr {
            Wptcr(0)
        }
    }
    impl core::fmt::Debug for Wptcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wptcr")
                .field("dcyc", &self.dcyc())
                .field("dhqc", &self.dhqc())
                .field("sshift", &self.sshift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wptcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wptcr {{ dcyc: {=u8:?}, dhqc: {=bool:?}, sshift: {=bool:?} }}",
                self.dcyc(),
                self.dhqc(),
                self.sshift()
            )
        }
    }
    #[doc = "XSPI write timing configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtcr(pub u32);
    impl Wtcr {
        #[doc = "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
        #[inline(always)]
        pub fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Wtcr {
        #[inline(always)]
        fn default() -> Wtcr {
            Wtcr(0)
        }
    }
    impl core::fmt::Debug for Wtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wtcr").field("dcyc", &self.dcyc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wtcr {{ dcyc: {=u8:?} }}", self.dcyc())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Apms {
        #[doc = "Automatic status-polling mode is stopped only by abort or by disabling the XSPI."]
        B_0X0 = 0x0,
        #[doc = "Automatic status-polling mode stops as soon as there is a match."]
        B_0X1 = 0x01,
    }
    impl Apms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Apms {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Apms {
        #[inline(always)]
        fn from(val: u8) -> Apms {
            Apms::from_bits(val)
        }
    }
    impl From<Apms> for u8 {
        #[inline(always)]
        fn from(val: Apms) -> u8 {
            Apms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrAbmode {
        #[doc = "no alternate bytes"]
        B_0X0 = 0x0,
        #[doc = "alternate bytes on a single line"]
        B_0X1 = 0x01,
        #[doc = "alternate bytes on two lines"]
        B_0X2 = 0x02,
        #[doc = "alternate bytes on four lines"]
        B_0X3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CcrAbmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrAbmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrAbmode {
        #[inline(always)]
        fn from(val: u8) -> CcrAbmode {
            CcrAbmode::from_bits(val)
        }
    }
    impl From<CcrAbmode> for u8 {
        #[inline(always)]
        fn from(val: CcrAbmode) -> u8 {
            CcrAbmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrAbsize {
        #[doc = "8-bit alternate bytes"]
        B_0X0 = 0x0,
        #[doc = "16-bit alternate bytes"]
        B_0X1 = 0x01,
        #[doc = "24-bit alternate bytes"]
        B_0X2 = 0x02,
        #[doc = "32-bit alternate bytes"]
        B_0X3 = 0x03,
    }
    impl CcrAbsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrAbsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrAbsize {
        #[inline(always)]
        fn from(val: u8) -> CcrAbsize {
            CcrAbsize::from_bits(val)
        }
    }
    impl From<CcrAbsize> for u8 {
        #[inline(always)]
        fn from(val: CcrAbsize) -> u8 {
            CcrAbsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrAdmode {
        #[doc = "no address"]
        B_0X0 = 0x0,
        #[doc = "address on a single line"]
        B_0X1 = 0x01,
        #[doc = "address on two lines"]
        B_0X2 = 0x02,
        #[doc = "address on four lines"]
        B_0X3 = 0x03,
        #[doc = "address on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CcrAdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrAdmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrAdmode {
        #[inline(always)]
        fn from(val: u8) -> CcrAdmode {
            CcrAdmode::from_bits(val)
        }
    }
    impl From<CcrAdmode> for u8 {
        #[inline(always)]
        fn from(val: CcrAdmode) -> u8 {
            CcrAdmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrAdsize {
        #[doc = "8-bit address"]
        B_0X0 = 0x0,
        #[doc = "16-bit address"]
        B_0X1 = 0x01,
        #[doc = "24-bit address"]
        B_0X2 = 0x02,
        #[doc = "32-bit address"]
        B_0X3 = 0x03,
    }
    impl CcrAdsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrAdsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrAdsize {
        #[inline(always)]
        fn from(val: u8) -> CcrAdsize {
            CcrAdsize::from_bits(val)
        }
    }
    impl From<CcrAdsize> for u8 {
        #[inline(always)]
        fn from(val: CcrAdsize) -> u8 {
            CcrAdsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrDmode {
        #[doc = "no data"]
        B_0X0 = 0x0,
        #[doc = "data on a single line"]
        B_0X1 = 0x01,
        #[doc = "data on two lines"]
        B_0X2 = 0x02,
        #[doc = "data on four lines"]
        B_0X3 = 0x03,
        #[doc = "data on eight lines"]
        B_0X4 = 0x04,
        #[doc = "data on 16 lines"]
        B_0X5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CcrDmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrDmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrDmode {
        #[inline(always)]
        fn from(val: u8) -> CcrDmode {
            CcrDmode::from_bits(val)
        }
    }
    impl From<CcrDmode> for u8 {
        #[inline(always)]
        fn from(val: CcrDmode) -> u8 {
            CcrDmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrImode {
        #[doc = "no instruction"]
        B_0X0 = 0x0,
        #[doc = "instruction on a single line"]
        B_0X1 = 0x01,
        #[doc = "instruction on two lines"]
        B_0X2 = 0x02,
        #[doc = "instruction on four lines"]
        B_0X3 = 0x03,
        #[doc = "instruction on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CcrImode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrImode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrImode {
        #[inline(always)]
        fn from(val: u8) -> CcrImode {
            CcrImode::from_bits(val)
        }
    }
    impl From<CcrImode> for u8 {
        #[inline(always)]
        fn from(val: CcrImode) -> u8 {
            CcrImode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcrIsize {
        #[doc = "8-bit instruction"]
        B_0X0 = 0x0,
        #[doc = "16-bit instruction"]
        B_0X1 = 0x01,
        #[doc = "24-bit instruction"]
        B_0X2 = 0x02,
        #[doc = "32-bit instruction"]
        B_0X3 = 0x03,
    }
    impl CcrIsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcrIsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcrIsize {
        #[inline(always)]
        fn from(val: u8) -> CcrIsize {
            CcrIsize::from_bits(val)
        }
    }
    impl From<CcrIsize> for u8 {
        #[inline(always)]
        fn from(val: CcrIsize) -> u8 {
            CcrIsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckmode {
        #[doc = "CLK must stay low while NCS is high (chip-select released), referred to as clock mode 0."]
        B_0X0 = 0x0,
        #[doc = "CLK must stay high while NCS is high (chip-select released), referred to as clock mode 3."]
        B_0X1 = 0x01,
    }
    impl Ckmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckmode {
        #[inline(always)]
        fn from(val: u8) -> Ckmode {
            Ckmode::from_bits(val)
        }
    }
    impl From<Ckmode> for u8 {
        #[inline(always)]
        fn from(val: Ckmode) -> u8 {
            Ckmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Csbound {
        #[doc = "NCS boundary disabled"]
        B_0X0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Csbound {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Csbound {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Csbound {
        #[inline(always)]
        fn from(val: u8) -> Csbound {
            Csbound::from_bits(val)
        }
    }
    impl From<Csbound> for u8 {
        #[inline(always)]
        fn from(val: Csbound) -> u8 {
            Csbound::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Csht {
        #[doc = "NCS stays high for at least 1 cycle between external device commands."]
        B_0X0 = 0x0,
        #[doc = "NCS stays high for at least 2 cycles between external device commands."]
        B_0X1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        #[doc = "NCS stays high for at least 64 cycles between external device commands."]
        B_0X3F = 0x3f,
    }
    impl Csht {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Csht {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Csht {
        #[inline(always)]
        fn from(val: u8) -> Csht {
            Csht::from_bits(val)
        }
    }
    impl From<Csht> for u8 {
        #[inline(always)]
        fn from(val: Csht) -> u8 {
            Csht::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cssel {
        #[doc = "NCS1 active"]
        B_0X0 = 0x0,
        #[doc = "NCS2 active"]
        B_0X1 = 0x01,
    }
    impl Cssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cssel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cssel {
        #[inline(always)]
        fn from(val: u8) -> Cssel {
            Cssel::from_bits(val)
        }
    }
    impl From<Cssel> for u8 {
        #[inline(always)]
        fn from(val: Cssel) -> u8 {
            Cssel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmode {
        #[doc = "indirect-write mode"]
        B_0X0 = 0x0,
        #[doc = "indirect-read mode"]
        B_0X1 = 0x01,
        #[doc = "automatic status-polling mode"]
        B_0X2 = 0x02,
        #[doc = "memory-mapped mode"]
        B_0X3 = 0x03,
    }
    impl Fmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmode {
        #[inline(always)]
        fn from(val: u8) -> Fmode {
            Fmode::from_bits(val)
        }
    }
    impl From<Fmode> for u8 {
        #[inline(always)]
        fn from(val: Fmode) -> u8 {
            Fmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fthres {
        #[doc = "FTF is set if there are one or more free bytes available to be written to in the FIFO"]
        B_0X0 = 0x0,
        #[doc = "FTF is set if there are two or more free bytes available to be written to in the FIFO"]
        B_0X1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        #[doc = "FTF is set if there are 64 free bytes available to be written to in the FIFO"]
        B_0X3F = 0x3f,
    }
    impl Fthres {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fthres {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fthres {
        #[inline(always)]
        fn from(val: u8) -> Fthres {
            Fthres::from_bits(val)
        }
    }
    impl From<Fthres> for u8 {
        #[inline(always)]
        fn from(val: Fthres) -> u8 {
            Fthres::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lm {
        #[doc = "Variable initial latency"]
        B_0X0 = 0x0,
        #[doc = "Fixed latency"]
        B_0X1 = 0x01,
    }
    impl Lm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lm {
        #[inline(always)]
        fn from(val: u8) -> Lm {
            Lm::from_bits(val)
        }
    }
    impl From<Lm> for u8 {
        #[inline(always)]
        fn from(val: Lm) -> u8 {
            Lm::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mask(u32);
    impl Mask {
        #[doc = "bit n of the data received in automatic status-polling mode is masked and its value is not considered in the matching logic."]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "bit n of the data received in automatic status-polling mode is unmasked and its value is considered in the matching logic."]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Mask {
        pub const fn from_bits(val: u32) -> Mask {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Mask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                0x01 => f.write_str("B_0X1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mask {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                0x01 => defmt::write!(f, "B_0X1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Mask {
        #[inline(always)]
        fn from(val: u32) -> Mask {
            Mask::from_bits(val)
        }
    }
    impl From<Mask> for u32 {
        #[inline(always)]
        fn from(val: Mask) -> u32 {
            Mask::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maxtran(u8);
    impl Maxtran {
        #[doc = "maximum communication disabled"]
        pub const B_0X0: Self = Self(0x0);
    }
    impl Maxtran {
        pub const fn from_bits(val: u8) -> Maxtran {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Maxtran {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maxtran {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Maxtran {
        #[inline(always)]
        fn from(val: u8) -> Maxtran {
            Maxtran::from_bits(val)
        }
    }
    impl From<Maxtran> for u8 {
        #[inline(always)]
        fn from(val: Maxtran) -> u8 {
            Maxtran::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msel {
        #[doc = "data exchanged over IO\\[3:0\\]"]
        B_0X0 = 0x0,
        #[doc = "data exchanged over IO\\[7:4\\]"]
        B_0X1 = 0x01,
        #[doc = "data exchanged over IO\\[11:8\\]"]
        B_0X2 = 0x02,
        #[doc = "data exchanged over IO\\[15:12\\]"]
        B_0X3 = 0x03,
    }
    impl Msel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msel {
        #[inline(always)]
        fn from(val: u8) -> Msel {
            Msel::from_bits(val)
        }
    }
    impl From<Msel> for u8 {
        #[inline(always)]
        fn from(val: Msel) -> u8 {
            Msel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mtyp {
        #[doc = "Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in single-, dual-, quad-, and octal-SPI modes."]
        B_0X0 = 0x0,
        #[doc = "Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in single-, dual-, quad-, and octal-SPI modes."]
        B_0X1 = 0x01,
        #[doc = "Standard mode"]
        B_0X2 = 0x02,
        #[doc = "Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in single-, dual-, quad-, and octal-SPI modes with dedicated address mapping."]
        B_0X3 = 0x03,
        #[doc = "HyperBus memory mode, the protocol follows the HyperBus<sup> </sup>specification. 8-data-bit DTR mode must be selected."]
        B_0X4 = 0x04,
        #[doc = "HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or indirect read/write modes must be used."]
        B_0X5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mtyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mtyp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mtyp {
        #[inline(always)]
        fn from(val: u8) -> Mtyp {
            Mtyp::from_bits(val)
        }
    }
    impl From<Mtyp> for u8 {
        #[inline(always)]
        fn from(val: Mtyp) -> u8 {
            Mtyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pmm {
        #[doc = "AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register."]
        B_0X0 = 0x0,
        #[doc = "OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register."]
        B_0X1 = 0x01,
    }
    impl Pmm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pmm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pmm {
        #[inline(always)]
        fn from(val: u8) -> Pmm {
            Pmm::from_bits(val)
        }
    }
    impl From<Pmm> for u8 {
        #[inline(always)]
        fn from(val: Pmm) -> u8 {
            Pmm::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Refresh(u32);
    impl Refresh {
        #[doc = "refresh disabled"]
        pub const B_0X0: Self = Self(0x0);
    }
    impl Refresh {
        pub const fn from_bits(val: u32) -> Refresh {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Refresh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B_0X0"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Refresh {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B_0X0"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Refresh {
        #[inline(always)]
        fn from(val: u32) -> Refresh {
            Refresh::from_bits(val)
        }
    }
    impl From<Refresh> for u32 {
        #[inline(always)]
        fn from(val: Refresh) -> u32 {
            Refresh::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrAbmode {
        #[doc = "no alternate bytes"]
        B_0X0 = 0x0,
        #[doc = "alternate bytes on a single line"]
        B_0X1 = 0x01,
        #[doc = "alternate bytes on two lines"]
        B_0X2 = 0x02,
        #[doc = "alternate bytes on four lines"]
        B_0X3 = 0x03,
        #[doc = "alternate bytes on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl WccrAbmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrAbmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrAbmode {
        #[inline(always)]
        fn from(val: u8) -> WccrAbmode {
            WccrAbmode::from_bits(val)
        }
    }
    impl From<WccrAbmode> for u8 {
        #[inline(always)]
        fn from(val: WccrAbmode) -> u8 {
            WccrAbmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrAbsize {
        #[doc = "8-bit alternate bytes"]
        B_0X0 = 0x0,
        #[doc = "16-bit alternate bytes"]
        B_0X1 = 0x01,
        #[doc = "24-bit alternate bytes"]
        B_0X2 = 0x02,
        #[doc = "32-bit alternate bytes"]
        B_0X3 = 0x03,
    }
    impl WccrAbsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrAbsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrAbsize {
        #[inline(always)]
        fn from(val: u8) -> WccrAbsize {
            WccrAbsize::from_bits(val)
        }
    }
    impl From<WccrAbsize> for u8 {
        #[inline(always)]
        fn from(val: WccrAbsize) -> u8 {
            WccrAbsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrAdmode {
        #[doc = "no address"]
        B_0X0 = 0x0,
        #[doc = "address on a single line"]
        B_0X1 = 0x01,
        #[doc = "address on two lines"]
        B_0X2 = 0x02,
        #[doc = "address on four lines"]
        B_0X3 = 0x03,
        #[doc = "address on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl WccrAdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrAdmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrAdmode {
        #[inline(always)]
        fn from(val: u8) -> WccrAdmode {
            WccrAdmode::from_bits(val)
        }
    }
    impl From<WccrAdmode> for u8 {
        #[inline(always)]
        fn from(val: WccrAdmode) -> u8 {
            WccrAdmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrAdsize {
        #[doc = "8-bit address"]
        B_0X0 = 0x0,
        #[doc = "16-bit address"]
        B_0X1 = 0x01,
        #[doc = "24-bit address"]
        B_0X2 = 0x02,
        #[doc = "32-bit address"]
        B_0X3 = 0x03,
    }
    impl WccrAdsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrAdsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrAdsize {
        #[inline(always)]
        fn from(val: u8) -> WccrAdsize {
            WccrAdsize::from_bits(val)
        }
    }
    impl From<WccrAdsize> for u8 {
        #[inline(always)]
        fn from(val: WccrAdsize) -> u8 {
            WccrAdsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrDmode {
        #[doc = "no data"]
        B_0X0 = 0x0,
        #[doc = "data on a single line"]
        B_0X1 = 0x01,
        #[doc = "data on two lines"]
        B_0X2 = 0x02,
        #[doc = "data on four lines"]
        B_0X3 = 0x03,
        #[doc = "data on eight lines"]
        B_0X4 = 0x04,
        #[doc = "Data on 16 lines"]
        B_0X5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "DATA reserved"]
        B_0X7 = 0x07,
    }
    impl WccrDmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrDmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrDmode {
        #[inline(always)]
        fn from(val: u8) -> WccrDmode {
            WccrDmode::from_bits(val)
        }
    }
    impl From<WccrDmode> for u8 {
        #[inline(always)]
        fn from(val: WccrDmode) -> u8 {
            WccrDmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrImode {
        #[doc = "no instruction"]
        B_0X0 = 0x0,
        #[doc = "instruction on a single line"]
        B_0X1 = 0x01,
        #[doc = "instruction on two lines"]
        B_0X2 = 0x02,
        #[doc = "instruction on four lines"]
        B_0X3 = 0x03,
        #[doc = "instruction on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl WccrImode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrImode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrImode {
        #[inline(always)]
        fn from(val: u8) -> WccrImode {
            WccrImode::from_bits(val)
        }
    }
    impl From<WccrImode> for u8 {
        #[inline(always)]
        fn from(val: WccrImode) -> u8 {
            WccrImode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WccrIsize {
        #[doc = "8-bit instruction"]
        B_0X0 = 0x0,
        #[doc = "16-bit instruction"]
        B_0X1 = 0x01,
        #[doc = "24-bit instruction"]
        B_0X2 = 0x02,
        #[doc = "32-bit instruction"]
        B_0X3 = 0x03,
    }
    impl WccrIsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WccrIsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WccrIsize {
        #[inline(always)]
        fn from(val: u8) -> WccrIsize {
            WccrIsize::from_bits(val)
        }
    }
    impl From<WccrIsize> for u8 {
        #[inline(always)]
        fn from(val: WccrIsize) -> u8 {
            WccrIsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrAbmode {
        #[doc = "no alternate bytes"]
        B_0X0 = 0x0,
        #[doc = "alternate bytes on a single line"]
        B_0X1 = 0x01,
        #[doc = "alternate bytes on two lines"]
        B_0X2 = 0x02,
        #[doc = "alternate bytes on four lines"]
        B_0X3 = 0x03,
        #[doc = "alternate bytes on eight lines"]
        B_0X4 = 0x04,
        #[doc = "alternate bytes on 16 lines"]
        B_0X5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "DATA reserved"]
        B_0X7 = 0x07,
    }
    impl WpccrAbmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrAbmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrAbmode {
        #[inline(always)]
        fn from(val: u8) -> WpccrAbmode {
            WpccrAbmode::from_bits(val)
        }
    }
    impl From<WpccrAbmode> for u8 {
        #[inline(always)]
        fn from(val: WpccrAbmode) -> u8 {
            WpccrAbmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrAbsize {
        #[doc = "8-bit alternate bytes"]
        B_0X0 = 0x0,
        #[doc = "16-bit alternate bytes"]
        B_0X1 = 0x01,
        #[doc = "24-bit alternate bytes"]
        B_0X2 = 0x02,
        #[doc = "32-bit alternate bytes"]
        B_0X3 = 0x03,
    }
    impl WpccrAbsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrAbsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrAbsize {
        #[inline(always)]
        fn from(val: u8) -> WpccrAbsize {
            WpccrAbsize::from_bits(val)
        }
    }
    impl From<WpccrAbsize> for u8 {
        #[inline(always)]
        fn from(val: WpccrAbsize) -> u8 {
            WpccrAbsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrAdmode {
        #[doc = "no address"]
        B_0X0 = 0x0,
        #[doc = "address on a single line"]
        B_0X1 = 0x01,
        #[doc = "address on two lines"]
        B_0X2 = 0x02,
        #[doc = "address on four lines"]
        B_0X3 = 0x03,
        #[doc = "address on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl WpccrAdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrAdmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrAdmode {
        #[inline(always)]
        fn from(val: u8) -> WpccrAdmode {
            WpccrAdmode::from_bits(val)
        }
    }
    impl From<WpccrAdmode> for u8 {
        #[inline(always)]
        fn from(val: WpccrAdmode) -> u8 {
            WpccrAdmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrAdsize {
        #[doc = "8-bit address"]
        B_0X0 = 0x0,
        #[doc = "16-bit address"]
        B_0X1 = 0x01,
        #[doc = "24-bit address"]
        B_0X2 = 0x02,
        #[doc = "32-bit address"]
        B_0X3 = 0x03,
    }
    impl WpccrAdsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrAdsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrAdsize {
        #[inline(always)]
        fn from(val: u8) -> WpccrAdsize {
            WpccrAdsize::from_bits(val)
        }
    }
    impl From<WpccrAdsize> for u8 {
        #[inline(always)]
        fn from(val: WpccrAdsize) -> u8 {
            WpccrAdsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrDmode {
        #[doc = "no data"]
        B_0X0 = 0x0,
        #[doc = "data on a single line"]
        B_0X1 = 0x01,
        #[doc = "data on two lines"]
        B_0X2 = 0x02,
        #[doc = "data on four lines"]
        B_0X3 = 0x03,
        #[doc = "data on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl WpccrDmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrDmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrDmode {
        #[inline(always)]
        fn from(val: u8) -> WpccrDmode {
            WpccrDmode::from_bits(val)
        }
    }
    impl From<WpccrDmode> for u8 {
        #[inline(always)]
        fn from(val: WpccrDmode) -> u8 {
            WpccrDmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrImode {
        #[doc = "no instruction"]
        B_0X0 = 0x0,
        #[doc = "instruction on a single line"]
        B_0X1 = 0x01,
        #[doc = "instruction on two lines"]
        B_0X2 = 0x02,
        #[doc = "instruction on four lines"]
        B_0X3 = 0x03,
        #[doc = "instruction on eight lines"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl WpccrImode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrImode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrImode {
        #[inline(always)]
        fn from(val: u8) -> WpccrImode {
            WpccrImode::from_bits(val)
        }
    }
    impl From<WpccrImode> for u8 {
        #[inline(always)]
        fn from(val: WpccrImode) -> u8 {
            WpccrImode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WpccrIsize {
        #[doc = "8-bit instruction"]
        B_0X0 = 0x0,
        #[doc = "16-bit instruction"]
        B_0X1 = 0x01,
        #[doc = "24-bit instruction"]
        B_0X2 = 0x02,
        #[doc = "32-bit instruction"]
        B_0X3 = 0x03,
    }
    impl WpccrIsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WpccrIsize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WpccrIsize {
        #[inline(always)]
        fn from(val: u8) -> WpccrIsize {
            WpccrIsize::from_bits(val)
        }
    }
    impl From<WpccrIsize> for u8 {
        #[inline(always)]
        fn from(val: WpccrIsize) -> u8 {
            WpccrIsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wrapsize {
        #[doc = "wrapped reads are not supported by the memory."]
        B_0X0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "external memory supports wrap size of 16 bytes."]
        B_0X2 = 0x02,
        #[doc = "external memory supports wrap size of 32 bytes."]
        B_0X3 = 0x03,
        #[doc = "external memory supports wrap size of 64 bytes."]
        B_0X4 = 0x04,
        #[doc = "external memory supports wrap size of 128 bytes."]
        B_0X5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Wrapsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wrapsize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wrapsize {
        #[inline(always)]
        fn from(val: u8) -> Wrapsize {
            Wrapsize::from_bits(val)
        }
    }
    impl From<Wrapsize> for u8 {
        #[inline(always)]
        fn from(val: Wrapsize) -> u8 {
            Wrapsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wzl {
        #[doc = "latency on write accesses"]
        B_0X0 = 0x0,
        #[doc = "no latency on write accesses"]
        B_0X1 = 0x01,
    }
    impl Wzl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wzl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wzl {
        #[inline(always)]
        fn from(val: u8) -> Wzl {
            Wzl::from_bits(val)
        }
    }
    impl From<Wzl> for u8 {
        #[inline(always)]
        fn from(val: Wzl) -> u8 {
            Wzl::to_bits(val)
        }
    }
}
