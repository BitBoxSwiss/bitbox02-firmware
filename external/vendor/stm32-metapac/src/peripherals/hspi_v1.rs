#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "HSPI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hspi {
    ptr: *mut u8,
}
unsafe impl Send for Hspi {}
unsafe impl Sync for Hspi {}
impl Hspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSPI control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "HSPI device configuration register 1."]
    #[inline(always)]
    pub const fn dcr1(self) -> crate::common::Reg<regs::Dcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "HSPI device configuration register 2."]
    #[inline(always)]
    pub const fn dcr2(self) -> crate::common::Reg<regs::Dcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "HSPI device configuration register 3."]
    #[inline(always)]
    pub const fn dcr3(self) -> crate::common::Reg<regs::Dcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "HSPI device configuration register 4."]
    #[inline(always)]
    pub const fn dcr4(self) -> crate::common::Reg<regs::Dcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "HSPI data length register."]
    #[inline(always)]
    pub const fn dlr(self) -> crate::common::Reg<regs::Dlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::Ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "HSPI polling status mask register."]
    #[inline(always)]
    pub const fn psmkr(self) -> crate::common::Reg<regs::Psmkr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "HSPI polling status match register."]
    #[inline(always)]
    pub const fn psmar(self) -> crate::common::Reg<regs::Psmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "HSPI polling interval register."]
    #[inline(always)]
    pub const fn pir(self) -> crate::common::Reg<regs::Pir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "HSPI communication configuration register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "HSPI timing configuration register."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "HSPI instruction register."]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "HSPI alternate bytes register."]
    #[inline(always)]
    pub const fn abr(self) -> crate::common::Reg<regs::Abr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "HSPI low-power timeout register."]
    #[inline(always)]
    pub const fn lptr(self) -> crate::common::Reg<regs::Lptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "HSPI wrap communication configuration register."]
    #[inline(always)]
    pub const fn wpccr(self) -> crate::common::Reg<regs::Wpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "HSPI wrap timing configuration register."]
    #[inline(always)]
    pub const fn wptcr(self) -> crate::common::Reg<regs::Wptcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "HSPI wrap instruction register."]
    #[inline(always)]
    pub const fn wpir(self) -> crate::common::Reg<regs::Wpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "HSPI wrap alternate bytes register."]
    #[inline(always)]
    pub const fn wpabr(self) -> crate::common::Reg<regs::Wpabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "HSPI write communication configuration register."]
    #[inline(always)]
    pub const fn wccr(self) -> crate::common::Reg<regs::Wccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "HSPI write timing configuration register."]
    #[inline(always)]
    pub const fn wtcr(self) -> crate::common::Reg<regs::Wtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "HSPI write instruction register."]
    #[inline(always)]
    pub const fn wir(self) -> crate::common::Reg<regs::Wir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "HSPI write alternate bytes register."]
    #[inline(always)]
    pub const fn wabr(self) -> crate::common::Reg<regs::Wabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "HSPI HyperBus latency configuration register."]
    #[inline(always)]
    pub const fn hlcr(self) -> crate::common::Reg<regs::Hlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "HSPI full-cycle calibration configuration."]
    #[inline(always)]
    pub const fn calfcr(self) -> crate::common::Reg<regs::Calfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "HSPI DLL master calibration configuration."]
    #[inline(always)]
    pub const fn calmr(self) -> crate::common::Reg<regs::Calmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "HSPI DLL slave output calibration configuration."]
    #[inline(always)]
    pub const fn calsor(self) -> crate::common::Reg<regs::Calsor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "HSPI DLL slave input calibration configuration."]
    #[inline(always)]
    pub const fn calsir(self) -> crate::common::Reg<regs::Calsir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
}
pub mod regs {
    #[doc = "HSPI alternate bytes register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Abr(pub u32);
    impl Abr {
        #[doc = "31: 0\\]: Alternate bytes Optional data to be send to the external SPI device right after the address."]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Alternate bytes Optional data to be send to the external SPI device right after the address."]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ar(pub u32);
    impl Ar {
        #[doc = "Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\\[0\\]
is forced to 1. Writes to this field are ignored when BUSY├é┬á=├é┬á1 or when FMODE = 11 (Memory-mapped mode)."]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\\[0\\]
is forced to 1. Writes to this field are ignored when BUSY├é┬á=├é┬á1 or when FMODE = 11 (Memory-mapped mode)."]
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
    #[doc = "HSPI full-cycle calibration configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfcr(pub u32);
    impl Calfcr {
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_coarse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case CALFCR and CALSR are updated with the values for the maximum delay."]
        #[inline(always)]
        pub const fn calmax(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case CALFCR and CALSR are updated with the values for the maximum delay."]
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
    #[doc = "HSPI DLL master calibration configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calmr(pub u32);
    impl Calmr {
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
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
    #[doc = "HSPI DLL slave input calibration configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calsir(pub u32);
    impl Calsir {
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
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
    #[doc = "HSPI DLL slave output calibration configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calsor(pub u32);
    impl Calsor {
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn fine(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub fn set_fine(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
        #[inline(always)]
        pub const fn coarse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
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
    #[doc = "HSPI communication configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn imode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_imode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        pub const fn isize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Instruction size This bit defines instruction size."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode This field defines the address phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn admode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Address mode This field defines the address phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_admode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
        pub const fn adsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation. 100-111: Reserved."]
        #[inline(always)]
        pub const fn abmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation. 100-111: Reserved."]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase. This field can be written only when BUSY├é┬á=├é┬á0."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double transfer rate This bit sets the DTR mode for the alternate bytes phase. This field can be written only when BUSY├é┬á=├é┬á0."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub const fn absize(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode This field defines the data phase mode of operation. 110-111: Reserved."]
        #[inline(always)]
        pub const fn dmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Data mode This field defines the data phase mode of operation. 110-111: Reserved."]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
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
        #[doc = "Send instruction only once mode This bit has no effect when IMODE├é┬á=├é┬á00 (see )."]
        #[inline(always)]
        pub const fn sioo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Send instruction only once mode This bit has no effect when IMODE├é┬á=├é┬á00 (see )."]
        #[inline(always)]
        pub fn set_sioo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sioo", &self.sioo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccr {{ imode: {=u8:?}, idtr: {=bool:?}, isize: {=u8:?}, admode: {=u8:?}, addtr: {=bool:?}, adsize: {=u8:?}, abmode: {=u8:?}, abdtr: {=bool:?}, absize: {=u8:?}, dmode: {=u8:?}, ddtr: {=bool:?}, dqse: {=bool:?}, sioo: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse () , self . sioo ())
        }
    }
    #[doc = "HSPI control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable This bit enables the HSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
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
        #[doc = "DMA enable In Indirect mode, the DMA can be used to input or output data via DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable In Indirect mode, the DMA can be used to input or output data via DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write this bit during DMA operation."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]├é┬á=├é┬á11) is selected. This bit enables the timeout counter."]
        #[inline(always)]
        pub const fn tcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout counter enable This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]├é┬á=├é┬á11) is selected. This bit enables the timeout counter."]
        #[inline(always)]
        pub fn set_tcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity."]
        #[inline(always)]
        pub const fn dmm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Dual-memory mode This bit activates the Dual-memory mode, where two external devices are used simultaneously to double the throughput and the capacity."]
        #[inline(always)]
        pub fn set_dmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Memory select This bit is the mirror of bit 30. Refer to the description of MSEL\\[1:0\\]
above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0."]
        #[inline(always)]
        pub const fn fsel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Memory select This bit is the mirror of bit 30. Refer to the description of MSEL\\[1:0\\]
above. This bit is set when 1 is written in bit 30 or bit 7. When this bit is set, both b30 and b7 are read as 1. This bit is reset when bit 30 and bit7 are set to 0. When this bit is reset, both bit 30 and bit7 are read as 0."]
        #[inline(always)]
        pub fn set_fsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in SR, to be set. ... Note: If DMAEN├é┬á=├é┬á1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[5:0\\]
value."]
        #[inline(always)]
        pub const fn fthres(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "FIFO threshold level This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in SR, to be set. ... Note: If DMAEN├é┬á=├é┬á1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[5:0\\]
value."]
        #[inline(always)]
        pub fn set_fthres(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
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
        #[doc = "Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match."]
        #[inline(always)]
        pub const fn apms(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic-polling mode stop This bit determines if the automatic polling is stopped after a match."]
        #[inline(always)]
        pub fn set_apms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode."]
        #[inline(always)]
        pub const fn pmm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Polling match mode This bit indicates which method must be used to determine a match during the Automatic-polling mode."]
        #[inline(always)]
        pub fn set_pmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Functional mode This field defines the HSPI functional mode of operation. If DMAEN├é┬á=├é┬á1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN├é┬á=├é┬á1, the DMA request signal automatically goes to inactive state."]
        #[inline(always)]
        pub const fn fmode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Functional mode This field defines the HSPI functional mode of operation. If DMAEN├é┬á=├é┬á1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN├é┬á=├é┬á1, the DMA request signal automatically goes to inactive state."]
        #[inline(always)]
        pub fn set_fmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in single├ó\u{80}\u{91}memory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO\\[7:0\\]
1x: data exchanged over IO\\[15:8\\]
These bits are ignored when in dual-octal configuration (data on 8 bits and DMM├é┬á=├é┬á1) or 16├ó\u{80}\u{91}bit configuration (data exchanged over IO\\[15:0\\])."]
        #[inline(always)]
        pub const fn msel(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Flash select These bits select the memory to be addressed in Single, Dual, Quad or Octal mode in single├ó\u{80}\u{91}memory configuration (when DMM = 0). - when in Quad mode: - when in Octal mode or Dual-quad mode: 0x: data exchanged over IO\\[7:0\\]
1x: data exchanged over IO\\[15:8\\]
These bits are ignored when in dual-octal configuration (data on 8 bits and DMM├é┬á=├é┬á1) or 16├ó\u{80}\u{91}bit configuration (data exchanged over IO\\[15:0\\])."]
        #[inline(always)]
        pub fn set_msel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
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
                .field("fsel", &self.fsel())
                .field("fthres", &self.fthres())
                .field("teie", &self.teie())
                .field("tcie", &self.tcie())
                .field("ftie", &self.ftie())
                .field("smie", &self.smie())
                .field("toie", &self.toie())
                .field("apms", &self.apms())
                .field("pmm", &self.pmm())
                .field("fmode", &self.fmode())
                .field("msel", &self.msel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, abort: {=bool:?}, dmaen: {=bool:?}, tcen: {=bool:?}, dmm: {=bool:?}, fsel: {=bool:?}, fthres: {=u8:?}, teie: {=bool:?}, tcie: {=bool:?}, ftie: {=bool:?}, smie: {=bool:?}, toie: {=bool:?}, apms: {=bool:?}, pmm: {=bool:?}, fmode: {=u8:?}, msel: {=u8:?} }}" , self . en () , self . abort () , self . dmaen () , self . tcen () , self . dmm () , self . fsel () , self . fthres () , self . teie () , self . tcie () , self . ftie () , self . smie () , self . toie () , self . apms () , self . pmm () , self . fmode () , self . msel ())
        }
    }
    #[doc = "HSPI device configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr1(pub u32);
    impl Dcr1 {
        #[doc = "Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCS├é┬á=├é┬á1)."]
        #[inline(always)]
        pub const fn ckmode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCS├é┬á=├é┬á1)."]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        #[doc = "Delay block bypass."]
        #[inline(always)]
        pub const fn dlybyp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Delay block bypass."]
        #[inline(always)]
        pub fn set_dlybyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Chip-select high time CSHT├é┬á+├é┬á1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT\\[5:3\\]
are reserved and the number of cycles is limited to eight (refer to implementation)."]
        #[inline(always)]
        pub const fn csht(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Chip-select high time CSHT├é┬á+├é┬á1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT\\[5:3\\]
are reserved and the number of cycles is limited to eight (refer to implementation)."]
        #[inline(always)]
        pub fn set_csht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4├é┬áGbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256├é┬áMbytes. In Regular-command mode, if DMM├é┬á=├é┬á1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
        #[inline(always)]
        pub const fn devsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4├é┬áGbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256├é┬áMbytes. In Regular-command mode, if DMM├é┬á=├é┬á1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
        #[inline(always)]
        pub fn set_devsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved."]
        #[inline(always)]
        pub const fn mtyp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved."]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
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
                .field("dlybyp", &self.dlybyp())
                .field("csht", &self.csht())
                .field("devsize", &self.devsize())
                .field("mtyp", &self.mtyp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dcr1 {{ ckmode: {=bool:?}, frck: {=bool:?}, dlybyp: {=bool:?}, csht: {=u8:?}, devsize: {=u8:?}, mtyp: {=u8:?} }}" , self . ckmode () , self . frck () , self . dlybyp () , self . csht () , self . devsize () , self . mtyp ())
        }
    }
    #[doc = "HSPI device configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr2(pub u32);
    impl Dcr2 {
        #[doc = "Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (value├é┬á+├é┬á1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50├é┬á%. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case CALOSR or CALISR have been written in the meantime. BUSY stays high during the whole calibration execution."]
        #[inline(always)]
        pub const fn prescaler(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clock prescaler This field defines the scaler factor for generating the CLK based on the kernel clock (value├é┬á+├é┬á1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50├é┬á%. The clock signal remains low one cycle longer than it stays high. Writing this field automatically starts a new calibration of high-speed interface DLL at the start of next transfer, except in case CALOSR or CALISR have been written in the meantime. BUSY stays high during the whole calibration execution."]
        #[inline(always)]
        pub fn set_prescaler(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the WPIR register. 110-111: Reserved."]
        #[inline(always)]
        pub const fn wrapsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Wrap size This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the WPIR register. 110-111: Reserved."]
        #[inline(always)]
        pub fn set_wrapsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
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
                "Dcr2 {{ prescaler: {=u8:?}, wrapsize: {=u8:?} }}",
                self.prescaler(),
                self.wrapsize()
            )
        }
    }
    #[doc = "HSPI device configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr3(pub u32);
    impl Dcr3 {
        #[doc = "Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes."]
        #[inline(always)]
        pub const fn maxtran(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Maximum transfer This field enables the communication regulation feature. The nCS is released every MAXTRAN+1 clock cycles when the other HSPI request the access to the bus. others: Maximum communication is set to MAXTRAN+1 bytes."]
        #[inline(always)]
        pub fn set_maxtran(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes."]
        #[inline(always)]
        pub const fn csbound(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "CS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The nCS is released on each boundary of 2CSBOUND bytes. others: CS boundary set to 2CSBOUND bytes."]
        #[inline(always)]
        pub fn set_csbound(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
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
                "Dcr3 {{ maxtran: {=u8:?}, csbound: {=u8:?} }}",
                self.maxtran(),
                self.csbound()
            )
        }
    }
    #[doc = "HSPI device configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr4(pub u32);
    impl Dcr4 {
        #[doc = "Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles."]
        #[inline(always)]
        pub const fn refresh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Refresh rate This field enables the refresh rate feature. The nCS is released every REFRESH+1 clock cycles for writes, and REFRESH+4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in single, dual or quad mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH+1 clock cycles."]
        #[inline(always)]
        pub fn set_refresh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            defmt::write!(f, "Dcr4 {{ refresh: {=u32:?} }}", self.refresh())
        }
    }
    #[doc = "HSPI data length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlr(pub u32);
    impl Dlr {
        #[doc = "31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Status-polling modes. A value not greater than three (indicating 4 bytes) must be used for status polling-mode. All 1's in Indirect mode means undefined length, where HSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE├é┬á=├é┬á0x1F. DL\\[0\\]
is stuck at 1 in Dual-memory mode (DMM├é┬á=├é┬á1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect when in Memory-mapped mode."]
        #[inline(always)]
        pub const fn dl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Status-polling modes. A value not greater than three (indicating 4 bytes) must be used for status polling-mode. All 1's in Indirect mode means undefined length, where HSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE├é┬á=├é┬á0x1F. DL\\[0\\]
is stuck at 1 in Dual-memory mode (DMM├é┬á=├é┬á1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect when in Memory-mapped mode."]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "31: 0\\]: Data Data to be sent/received to/from the external SPI device In Indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In Indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY├é┬á=├é┬á1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In Automatic-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In Indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in Indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2├é┬ábytes, and a word read 4├é┬ábytes. Accesses in Indirect mode must be aligned to the bottom of this register: A byte read must read DATA\\[7:0\\]
and a half-word read must read DATA\\[15:0\\]."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Data Data to be sent/received to/from the external SPI device In Indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In Indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY├é┬á=├é┬á1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In Automatic-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In Indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in Indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2├é┬ábytes, and a word read 4├é┬ábytes. Accesses in Indirect mode must be aligned to the bottom of this register: A byte read must read DATA\\[7:0\\]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Clear transfer error flag Writing 1 clears the TEF flag in the SR register."]
        #[inline(always)]
        pub const fn ctef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer error flag Writing 1 clears the TEF flag in the SR register."]
        #[inline(always)]
        pub fn set_ctef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear transfer complete flag Writing 1 clears the TCF flag in the SR register."]
        #[inline(always)]
        pub const fn ctcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer complete flag Writing 1 clears the TCF flag in the SR register."]
        #[inline(always)]
        pub fn set_ctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear status match flag Writing 1 clears the SMF flag in the SR register."]
        #[inline(always)]
        pub const fn csmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear status match flag Writing 1 clears the SMF flag in the SR register."]
        #[inline(always)]
        pub fn set_csmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear timeout flag Writing 1 clears the TOF flag in the SR register."]
        #[inline(always)]
        pub const fn ctof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear timeout flag Writing 1 clears the TOF flag in the SR register."]
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
    #[doc = "HSPI HyperBus latency configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hlcr(pub u32);
    impl Hlcr {
        #[doc = "Latency mode This bit selects the Latency mode."]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Latency mode This bit selects the Latency mode."]
        #[inline(always)]
        pub fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write zero latency This bit enables zero latency on write operations."]
        #[inline(always)]
        pub const fn wzl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write zero latency This bit enables zero latency on write operations."]
        #[inline(always)]
        pub fn set_wzl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "7: 0\\]: Access time Device access time expressed in number of communication clock cycles."]
        #[inline(always)]
        pub const fn tacc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "7: 0\\]: Access time Device access time expressed in number of communication clock cycles."]
        #[inline(always)]
        pub fn set_tacc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Read write recovery time Device read write recovery time expressed in number of communication clock cycles."]
        #[inline(always)]
        pub const fn trwr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Read write recovery time Device read write recovery time expressed in number of communication clock cycles."]
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
                "Hlcr {{ lm: {=bool:?}, wzl: {=bool:?}, tacc: {=u8:?}, trwr: {=u8:?} }}",
                self.lm(),
                self.wzl(),
                self.tacc(),
                self.trwr()
            )
        }
    }
    #[doc = "HSPI instruction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Instruction Instruction to be sent to the external SPI device."]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Instruction Instruction to be sent to the external SPI device."]
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
    #[doc = "HSPI low-power timeout register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lptr(pub u32);
    impl Lptr {
        #[doc = "15: 0\\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state."]
        #[inline(always)]
        pub const fn timeout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "15: 0\\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state."]
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
    #[doc = "HSPI polling interval register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pir(pub u32);
    impl Pir {
        #[doc = "15: 0\\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases."]
        #[inline(always)]
        pub const fn interval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "15: 0\\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases."]
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
    #[doc = "HSPI polling status match register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psmar(pub u32);
    impl Psmar {
        #[doc = "31: 0\\]: Status match Value to be compared with the masked status register to get a match."]
        #[inline(always)]
        pub const fn match_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Status match Value to be compared with the masked status register to get a match."]
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
    #[doc = "HSPI polling status mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psmkr(pub u32);
    impl Psmkr {
        #[doc = "Status mask Mask to be applied to the status bytes received in Polling mode For bit n:."]
        #[inline(always)]
        pub const fn mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Status mask Mask to be applied to the status bytes received in Polling mode For bit n:."]
        #[inline(always)]
        pub fn set_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            defmt::write!(f, "Psmkr {{ mask: {=u32:?} }}", self.mask())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Transfer error flag This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF."]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error flag This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF."]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete flag This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete flag This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic-polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic-polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
        #[inline(always)]
        pub fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Status match flag This bit is set in Automatic-polling mode when the unmasked received data matches the corresponding bits in the match register (PSMAR). It is cleared by writing 1 to CSMF."]
        #[inline(always)]
        pub const fn smf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Status match flag This bit is set in Automatic-polling mode when the unmasked received data matches the corresponding bits in the match register (PSMAR). It is cleared by writing 1 to CSMF."]
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
        #[doc = "FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL├é┬á=├é┬á0 when the FIFO is empty, and 64 when it is full. In Automatic-status polling mode, FLEVEL is zero."]
        #[inline(always)]
        pub const fn flevel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "FIFO level This field gives the number of valid bytes that are being held in the FIFO. FLEVEL├é┬á=├é┬á0 when the FIFO is empty, and 64 when it is full. In Automatic-status polling mode, FLEVEL is zero."]
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
    #[doc = "HSPI timing configuration register."]
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
        #[doc = "Delay hold quarter cycle."]
        #[inline(always)]
        pub const fn dhqc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Delay hold quarter cycle."]
        #[inline(always)]
        pub fn set_dhqc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT├é┬á=├é┬á0 when the data phase is configured in DTR mode (when DDTR├é┬á=├é┬á1.)."]
        #[inline(always)]
        pub const fn sshift(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT├é┬á=├é┬á0 when the data phase is configured in DTR mode (when DDTR├é┬á=├é┬á1.)."]
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
    #[doc = "HSPI write alternate bytes register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wabr(pub u32);
    impl Wabr {
        #[doc = "31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
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
    #[doc = "HSPI write communication configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wccr(pub u32);
    impl Wccr {
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn imode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_imode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "Instruction size This bit defines instruction size:."]
        #[inline(always)]
        pub const fn isize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Instruction size This bit defines instruction size:."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode This field defines the address phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn admode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Address mode This field defines the address phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_admode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
        pub const fn adsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn abmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
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
        #[doc = "Alternate bytes size This field defines alternate bytes size:."]
        #[inline(always)]
        pub const fn absize(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Alternate bytes size This field defines alternate bytes size:."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode This field defines the data phase mode of operation."]
        #[inline(always)]
        pub const fn dmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Data mode This field defines the data phase mode of operation."]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
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
            defmt :: write ! (f , "Wccr {{ imode: {=u8:?}, idtr: {=bool:?}, isize: {=u8:?}, admode: {=u8:?}, addtr: {=bool:?}, adsize: {=u8:?}, abmode: {=u8:?}, abdtr: {=bool:?}, absize: {=u8:?}, dmode: {=u8:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse ())
        }
    }
    #[doc = "HSPI write instruction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wir(pub u32);
    impl Wir {
        #[doc = "Instruction Instruction to be sent to the external SPI device."]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Instruction Instruction to be sent to the external SPI device."]
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
    #[doc = "HSPI wrap alternate bytes register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpabr(pub u32);
    impl Wpabr {
        #[doc = "31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address."]
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
    #[doc = "HSPI wrap communication configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpccr(pub u32);
    impl Wpccr {
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn imode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_imode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        pub const fn isize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Instruction size This field defines instruction size."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode This field defines the address phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub const fn admode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Address mode This field defines the address phase mode of operation. 101-111: Reserved."]
        #[inline(always)]
        pub fn set_admode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
        pub const fn adsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Address size This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation."]
        #[inline(always)]
        pub const fn abmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Alternate-byte mode This field defines the alternate byte phase mode of operation."]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
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
        pub const fn absize(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Alternate bytes size This bit defines alternate bytes size."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode This field defines the data phase mode of operation. 101; Data on 16 lines 110-111: Reserved."]
        #[inline(always)]
        pub const fn dmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Data mode This field defines the data phase mode of operation. 101; Data on 16 lines 110-111: Reserved."]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
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
            defmt :: write ! (f , "Wpccr {{ imode: {=u8:?}, idtr: {=bool:?}, isize: {=u8:?}, admode: {=u8:?}, addtr: {=bool:?}, adsize: {=u8:?}, abmode: {=u8:?}, abdtr: {=bool:?}, absize: {=u8:?}, dmode: {=u8:?}, ddtr: {=bool:?}, dqse: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse ())
        }
    }
    #[doc = "HSPI wrap instruction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpir(pub u32);
    impl Wpir {
        #[doc = "31: 0\\]: Instruction Instruction to be sent to the external SPI device."]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Instruction Instruction to be sent to the external SPI device."]
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
    #[doc = "HSPI wrap timing configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wptcr(pub u32);
    impl Wptcr {
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
        #[doc = "Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR├é┬á=├é┬á1)."]
        #[inline(always)]
        pub const fn sshift(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR├é┬á=├é┬á1)."]
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
    #[doc = "HSPI write timing configuration register."]
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
