#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OctoSPI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Octospi {
    ptr: *mut u8,
}
unsafe impl Send for Octospi {}
unsafe impl Sync for Octospi {}
impl Octospi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "device configuration register 1"]
    #[inline(always)]
    pub const fn dcr1(self) -> crate::common::Reg<regs::Dcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "device configuration register 2"]
    #[inline(always)]
    pub const fn dcr2(self) -> crate::common::Reg<regs::Dcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "device configuration register 3"]
    #[inline(always)]
    pub const fn dcr3(self) -> crate::common::Reg<regs::Dcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "device configuration register 4"]
    #[inline(always)]
    pub const fn dcr4(self) -> crate::common::Reg<regs::Dcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "flag clear register"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "data length register"]
    #[inline(always)]
    pub const fn dlr(self) -> crate::common::Reg<regs::Dlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "address register"]
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::Ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "polling status mask register"]
    #[inline(always)]
    pub const fn psmkr(self) -> crate::common::Reg<regs::Psmkr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "polling status match register"]
    #[inline(always)]
    pub const fn psmar(self) -> crate::common::Reg<regs::Psmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "polling interval register"]
    #[inline(always)]
    pub const fn pir(self) -> crate::common::Reg<regs::Pir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "communication configuration register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "timing configuration register"]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "instruction register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "alternate bytes register"]
    #[inline(always)]
    pub const fn abr(self) -> crate::common::Reg<regs::Abr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "low-power timeout register"]
    #[inline(always)]
    pub const fn lptr(self) -> crate::common::Reg<regs::Lptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "wrap communication configuration register"]
    #[inline(always)]
    pub const fn wpccr(self) -> crate::common::Reg<regs::Wpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "wrap timing configuration register"]
    #[inline(always)]
    pub const fn wptcr(self) -> crate::common::Reg<regs::Wptcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "wrap instruction register"]
    #[inline(always)]
    pub const fn wpir(self) -> crate::common::Reg<regs::Wpir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "wrap alternate bytes register"]
    #[inline(always)]
    pub const fn wpabr(self) -> crate::common::Reg<regs::Wpabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "write communication configuration register"]
    #[inline(always)]
    pub const fn wccr(self) -> crate::common::Reg<regs::Wccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "write timing configuration register"]
    #[inline(always)]
    pub const fn wtcr(self) -> crate::common::Reg<regs::Wtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "write instruction register"]
    #[inline(always)]
    pub const fn wir(self) -> crate::common::Reg<regs::Wir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "write alternate bytes register"]
    #[inline(always)]
    pub const fn wabr(self) -> crate::common::Reg<regs::Wabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "OCTOSPI HyperBus latency configuration register"]
    #[inline(always)]
    pub const fn hlcr(self) -> crate::common::Reg<regs::Hlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
}
pub mod regs {
    #[doc = "alternate bytes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Abr(pub u32);
    impl Abr {
        #[doc = "Alternate bytes"]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alternate bytes"]
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
    #[doc = "address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ar(pub u32);
    impl Ar {
        #[doc = "Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 1. Writes to. this field are ignored when BUSY = 1 or when FMODE = 11 (Memory-mapped mode)."]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 1. Writes to. this field are ignored when BUSY = 1 or when FMODE = 11 (Memory-mapped mode)."]
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
    #[doc = "communication configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Instruction mode. This field defines the instruction phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn imode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Instruction mode. This field defines the instruction phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_imode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Instruction double transfer rate. This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction double transfer rate. This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Instruction size. This bit defines instruction size."]
        #[inline(always)]
        pub const fn isize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Instruction size. This bit defines instruction size."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode. This field defines the address phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn admode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Address mode. This field defines the address phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_admode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Address double transfer rate. This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address double transfer rate. This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address size. This field defines address size."]
        #[inline(always)]
        pub const fn adsize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Address size. This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode. This field defines the alternate-byte phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn abmode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Alternate-byte mode. This field defines the alternate-byte phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double transfer rate. This bit sets the DTR mode for the alternate bytes phase. This field can be written only when BUSY = 0."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double transfer rate. This bit sets the DTR mode for the alternate bytes phase. This field can be written only when BUSY = 0."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size. This bit defines alternate bytes size."]
        #[inline(always)]
        pub const fn absize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Alternate bytes size. This bit defines alternate bytes size."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode. This field defines the data phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn dmode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Data mode. This field defines the data phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Data double transfer rate. This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Data double transfer rate. This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DQS enable. This bit enables the data strobe management."]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DQS enable. This bit enables the data strobe management."]
        #[inline(always)]
        pub fn set_dqse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Send instruction only once mode. This bit has no effect when IMODE = 00 (see )."]
        #[inline(always)]
        pub const fn sioo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Send instruction only once mode. This bit has no effect when IMODE = 00 (see )."]
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
            defmt :: write ! (f , "Ccr {{ imode: {:?}, idtr: {=bool:?}, isize: {:?}, admode: {:?}, addtr: {=bool:?}, adsize: {:?}, abmode: {:?}, abdtr: {=bool:?}, absize: {:?}, dmode: {:?}, ddtr: {=bool:?}, dqse: {=bool:?}, sioo: {=bool:?} }}" , self . imode () , self . idtr () , self . isize () , self . admode () , self . addtr () , self . adsize () , self . abmode () , self . abdtr () , self . absize () , self . dmode () , self . ddtr () , self . dqse () , self . sioo ())
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable This bit enables the OCTOSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case. this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable This bit enables the OCTOSPI. Note: The DMA request can be aborted without having received the ACK in case this EN bit is cleared during the operation. In case. this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to inactive state without waiting for the ACK signal from DMA to be active."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Abort request. This bit aborts the ongoing command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
        #[inline(always)]
        pub const fn abort(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Abort request. This bit aborts the ongoing command sequence. It is automatically reset once the abort is completed. This bit stops the current transfer. Note: This bit is always read as 0."]
        #[inline(always)]
        pub fn set_abort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA enable In Indirect mode, the DMA can be used to input or output data via DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write. this bit during DMA operation."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable In Indirect mode, the DMA can be used to input or output data via DR. DMA transfers are initiated when FTF is set. Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake with the DMA. Do not write. this bit during DMA operation."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timeout counter enable. This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter."]
        #[inline(always)]
        pub const fn tcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout counter enable. This bit is valid only when the Memory-mapped mode (FMODE\\[1:0\\]
= 11) is selected. This bit enables the timeout counter."]
        #[inline(always)]
        pub fn set_tcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Dual-memory configuration. This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity"]
        #[inline(always)]
        pub const fn dmm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Dual-memory configuration. This bit activates the dual-memory configuration, where two external devices are used simultaneously to double the throughput and the capacity"]
        #[inline(always)]
        pub fn set_dmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Flash select. This bit selects the Flash memory to be addressed in Single-, Dual-, Quad-SPI mode in single-memory configuration (when DMM = 0). This bit is ignored when DMM = 1 or when Octal-SPI mode is selected."]
        #[inline(always)]
        pub const fn fsel(&self) -> super::vals::FlashSelect {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::FlashSelect::from_bits(val as u8)
        }
        #[doc = "Flash select. This bit selects the Flash memory to be addressed in Single-, Dual-, Quad-SPI mode in single-memory configuration (when DMM = 0). This bit is ignored when DMM = 1 or when Octal-SPI mode is selected."]
        #[inline(always)]
        pub fn set_fsel(&mut self, val: super::vals::FlashSelect) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "FIFO threshold level. This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[4:0\\]
value."]
        #[inline(always)]
        pub const fn fthres(&self) -> super::vals::Threshold {
            let val = (self.0 >> 8usize) & 0x1f;
            super::vals::Threshold::from_bits(val as u8)
        }
        #[doc = "FIFO threshold level. This field defines, in Indirect mode, the threshold number of bytes in the FIFO that causes the FIFO threshold flag FTF in SR, to be set. ... Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled before changing the FTHRES\\[4:0\\]
value."]
        #[inline(always)]
        pub fn set_fthres(&mut self, val: super::vals::Threshold) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
        }
        #[doc = "Transfer error interrupt enable. This bit enables the transfer error interrupt."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable. This bit enables the transfer error interrupt."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Transfer complete interrupt enable. This bit enables the transfer complete interrupt."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable. This bit enables the transfer complete interrupt."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "FIFO threshold interrupt enable. This bit enables the FIFO threshold interrupt."]
        #[inline(always)]
        pub const fn ftie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO threshold interrupt enable. This bit enables the FIFO threshold interrupt."]
        #[inline(always)]
        pub fn set_ftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Status match interrupt enable. This bit enables the status match interrupt."]
        #[inline(always)]
        pub const fn smie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Status match interrupt enable. This bit enables the status match interrupt."]
        #[inline(always)]
        pub fn set_smie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timeout interrupt enable. This bit enables the timeout interrupt."]
        #[inline(always)]
        pub const fn toie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout interrupt enable. This bit enables the timeout interrupt."]
        #[inline(always)]
        pub fn set_toie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Automatic status-polling mode stop. This bit determines if the Automatic status-polling mode is stopped after a match."]
        #[inline(always)]
        pub const fn apms(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic status-polling mode stop. This bit determines if the Automatic status-polling mode is stopped after a match."]
        #[inline(always)]
        pub fn set_apms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Polling match mode. This bit indicates which method must be used to determine a match during the Automatic status-polling mode."]
        #[inline(always)]
        pub const fn pmm(&self) -> super::vals::MatchMode {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::MatchMode::from_bits(val as u8)
        }
        #[doc = "Polling match mode. This bit indicates which method must be used to determine a match during the Automatic status-polling mode."]
        #[inline(always)]
        pub fn set_pmm(&mut self, val: super::vals::MatchMode) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Functional mode. This field defines the OCTOSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state."]
        #[inline(always)]
        pub const fn fmode(&self) -> super::vals::FunctionalMode {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::FunctionalMode::from_bits(val as u8)
        }
        #[doc = "Functional mode. This field defines the OCTOSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE\\[1:0\\]
value. If FMODE\\[1:0\\]
and FTHRES\\[4:0\\]
are wrongly updated while DMAEN = 1, the DMA request signal automatically goes to inactive state."]
        #[inline(always)]
        pub fn set_fmode(&mut self, val: super::vals::FunctionalMode) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ en: {=bool:?}, abort: {=bool:?}, dmaen: {=bool:?}, tcen: {=bool:?}, dmm: {=bool:?}, fsel: {:?}, fthres: {:?}, teie: {=bool:?}, tcie: {=bool:?}, ftie: {=bool:?}, smie: {=bool:?}, toie: {=bool:?}, apms: {=bool:?}, pmm: {:?}, fmode: {:?} }}" , self . en () , self . abort () , self . dmaen () , self . tcen () , self . dmm () , self . fsel () , self . fthres () , self . teie () , self . tcie () , self . ftie () , self . smie () , self . toie () , self . apms () , self . pmm () , self . fmode ())
        }
    }
    #[doc = "device configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr1(pub u32);
    impl Dcr1 {
        #[doc = "Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
        #[inline(always)]
        pub const fn ckmode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1)."]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Free running clock. This bit configures the free running clock."]
        #[inline(always)]
        pub const fn frck(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Free running clock. This bit configures the free running clock."]
        #[inline(always)]
        pub fn set_frck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Delay block bypass"]
        #[inline(always)]
        pub const fn dlybyp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Delay block bypass"]
        #[inline(always)]
        pub fn set_dlybyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
        #[inline(always)]
        pub const fn csht(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ..."]
        #[inline(always)]
        pub fn set_csht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Device size. This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
        #[inline(always)]
        pub const fn devsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Device size. This field defines the size of the external device using the following formula: Number of bytes in device = 2\\[DEVSIZE+1\\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\\[4:0\\]
indicates the total capacity of the two devices together."]
        #[inline(always)]
        pub fn set_devsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Memory type. This bit indicates the type of memory to be supported. Note: In. this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::MemType {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::MemType::from_bits(val as u8)
        }
        #[doc = "Memory type. This bit indicates the type of memory to be supported. Note: In. this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\\[2:0\\]
for memories different from Micron. Others: Reserved"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::MemType) {
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
            defmt :: write ! (f , "Dcr1 {{ ckmode: {=bool:?}, frck: {=bool:?}, dlybyp: {=bool:?}, csht: {=u8:?}, devsize: {=u8:?}, mtyp: {:?} }}" , self . ckmode () , self . frck () , self . dlybyp () , self . csht () , self . devsize () , self . mtyp ())
        }
    }
    #[doc = "device configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr2(pub u32);
    impl Dcr2 {
        #[doc = "Clock prescaler. This field defines the scaler factor for generating the CLK based on the kernel clock (value + 1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50 %. The clock signal remains low one cycle longer than it stays high."]
        #[inline(always)]
        pub const fn prescaler(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clock prescaler. This field defines the scaler factor for generating the CLK based on the kernel clock (value + 1). 2: FCLK = FKERNEL/3 ... 255: FCLK = FKERNEL/256 For odd clock division factors, the CLK duty cycle is not 50 %. The clock signal remains low one cycle longer than it stays high."]
        #[inline(always)]
        pub fn set_prescaler(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Wrap size. This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the OCTOSPI1_WPIR register. 110-111: Reserved"]
        #[inline(always)]
        pub const fn wrapsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Wrap size. This field indicates the wrap size to which the memory is configured. For memories which have a separate command for wrapped instructions, this field indicates the wrap-size associated with the command held in the OCTOSPI1_WPIR register. 110-111: Reserved"]
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
    #[doc = "device configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr3(pub u32);
    impl Dcr3 {
        #[doc = "Maximum transfer"]
        #[inline(always)]
        pub const fn maxtran(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Maximum transfer"]
        #[inline(always)]
        pub fn set_maxtran(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "NCS boundary. This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
        #[inline(always)]
        pub const fn csbound(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "NCS boundary. This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
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
    #[doc = "device configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr4(pub u32);
    impl Dcr4 {
        #[doc = "Refresh rate. This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
        #[inline(always)]
        pub const fn refresh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Refresh rate. This field enables the refresh rate feature. The NCS is released every REFRESH + 1 clock cycles for writes, and REFRESH + 4 clock cycles for reads. Note: These two values can be extended with few clock cycles when refresh occurs during a byte transmission in Single-, Dual- or Quad-SPI mode, because the byte transmission must be completed. others: Maximum communication length is set to REFRESH + 1 clock cycles."]
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
    #[doc = "data length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlr(pub u32);
    impl Dlr {
        #[doc = "31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Automatic status-polling modes. A value not greater than three (indicating 4 bytes) must be used for Automatic status-polling mode. All 1’s in Indirect mode means undefined length, where OCTOSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F. DL\\[0\\]
is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to. this bit, thus assuring that each access transfers an even number of bytes. This field has no effect in Memory-mapped mode."]
        #[inline(always)]
        pub const fn dl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Automatic status-polling modes. A value not greater than three (indicating 4 bytes) must be used for Automatic status-polling mode. All 1’s in Indirect mode means undefined length, where OCTOSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F. DL\\[0\\]
is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to. this bit, thus assuring that each access transfers an even number of bytes. This field has no effect in Memory-mapped mode."]
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
    #[doc = "data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "31: 0\\]: Data Data to be sent/received to/from the external SPI device In Indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In Indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY = 1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In Automatic status-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In Indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in Indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2 bytes, and a word read 4 bytes. Accesses in Indirect mode must be aligned to the bottom of. this register: A byte read must read DATA\\[7:0\\]
and a half-word read must read DATA\\[15:0\\]."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Data Data to be sent/received to/from the external SPI device In Indirect-write mode, data written to this register is stored on the FIFO before it is sent to the external device during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In Indirect-read mode, reading this register gives (via the FIFO) the data that was received from the external device. If the FIFO does not have as many bytes as requested by the read operation and if BUSY = 1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In Automatic status-polling mode, this register contains the last data read from the external device (without masking). Word, half-word, and byte accesses to this register are supported. In Indirect-write mode, a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes. Similarly, in Indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read 2 bytes, and a word read 4 bytes. Accesses in Indirect mode must be aligned to the bottom of. this register: A byte read must read DATA\\[7:0\\]
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
    #[doc = "flag clear register"]
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
    #[doc = "OCTOSPI HyperBus latency configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hlcr(pub u32);
    impl Hlcr {
        #[doc = "Latency mode. This bit selects the Latency mode."]
        #[inline(always)]
        pub const fn lm(&self) -> super::vals::LatencyMode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::LatencyMode::from_bits(val as u8)
        }
        #[doc = "Latency mode. This bit selects the Latency mode."]
        #[inline(always)]
        pub fn set_lm(&mut self, val: super::vals::LatencyMode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Write zero latency. This bit enables zero latency on write operations."]
        #[inline(always)]
        pub const fn wzl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write zero latency. This bit enables zero latency on write operations."]
        #[inline(always)]
        pub fn set_wzl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "7: 0\\]: Access time. Device access time expressed in number of communication clock cycles"]
        #[inline(always)]
        pub const fn tacc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "7: 0\\]: Access time. Device access time expressed in number of communication clock cycles"]
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
                "Hlcr {{ lm: {:?}, wzl: {=bool:?}, tacc: {=u8:?}, trwr: {=u8:?} }}",
                self.lm(),
                self.wzl(),
                self.tacc(),
                self.trwr()
            )
        }
    }
    #[doc = "instruction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Instruction to be sent to the external SPI device"]
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
    #[doc = "low-power timeout register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lptr(pub u32);
    impl Lptr {
        #[doc = "15: 0\\]: Timeout period After each access in Memory-mapped mode, the OCTOSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the OCTOSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
        #[inline(always)]
        pub const fn timeout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "15: 0\\]: Timeout period After each access in Memory-mapped mode, the OCTOSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the OCTOSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
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
    #[doc = "polling interval register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pir(pub u32);
    impl Pir {
        #[doc = "15: 0\\]: Polling interval Number of CLK cycle between a read during the Automatic status-polling phases"]
        #[inline(always)]
        pub const fn interval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "15: 0\\]: Polling interval Number of CLK cycle between a read during the Automatic status-polling phases"]
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
    #[doc = "polling status match register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psmar(pub u32);
    impl Psmar {
        #[doc = "31: 0\\]: Status match Value to be compared with the masked status register to get a match"]
        #[inline(always)]
        pub const fn match_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Status match Value to be compared with the masked status register to get a match"]
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
    #[doc = "polling status mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psmkr(pub u32);
    impl Psmkr {
        #[doc = "Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
        #[inline(always)]
        pub const fn mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
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
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Transfer error flag. This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF."]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error flag. This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode. It is cleared by writing 1 to CTEF."]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete flag. This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete flag. This bit is set in Indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic status-polling mode, this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO threshold flag In Indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after the reads from the external device are complete. It is cleared automatically as soon as the threshold condition is no longer true. In Automatic status-polling mode, this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
        #[inline(always)]
        pub fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Status match flag. This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (PSMAR). It is cleared by writing 1 to CSMF."]
        #[inline(always)]
        pub const fn smf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Status match flag. This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (PSMAR). It is cleared by writing 1 to CSMF."]
        #[inline(always)]
        pub fn set_smf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timeout flag. This bit is set when timeout occurs. It is cleared by writing 1 to CTOF."]
        #[inline(always)]
        pub const fn tof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout flag. This bit is set when timeout occurs. It is cleared by writing 1 to CTOF."]
        #[inline(always)]
        pub fn set_tof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Busy. This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Busy. This bit is set when an operation is ongoing. It is cleared automatically when the operation with the external device is finished and the FIFO is empty."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO level. This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 32 when it is full. In Automatic status-polling mode, FLEVEL is zero."]
        #[inline(always)]
        pub const fn flevel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "FIFO level. This field gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 32 when it is full. In Automatic status-polling mode, FLEVEL is zero."]
        #[inline(always)]
        pub fn set_flevel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
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
    #[doc = "timing configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcr(pub u32);
    impl Tcr {
        #[doc = "Number of dummy cycles. This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of dummy cycles. This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
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
        #[doc = "Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
        #[inline(always)]
        pub const fn sshift(&self) -> super::vals::SampleShift {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::SampleShift::from_bits(val as u8)
        }
        #[doc = "Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
        #[inline(always)]
        pub fn set_sshift(&mut self, val: super::vals::SampleShift) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
                "Tcr {{ dcyc: {=u8:?}, dhqc: {=bool:?}, sshift: {:?} }}",
                self.dcyc(),
                self.dhqc(),
                self.sshift()
            )
        }
    }
    #[doc = "write alternate bytes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wabr(pub u32);
    impl Wabr {
        #[doc = "31: 0\\]: Alternate bytes. Optional data to be sent to the external SPI device right after the address"]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Alternate bytes. Optional data to be sent to the external SPI device right after the address"]
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
    #[doc = "OCTOSPI write communication configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wccr(pub u32);
    impl Wccr {
        #[doc = "Instruction mode. This field defines the instruction phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn imode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Instruction mode. This field defines the instruction phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_imode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Instruction double transfer rate. This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction double transfer rate. This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Instruction size. This bit defines instruction size:"]
        #[inline(always)]
        pub const fn isize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Instruction size. This bit defines instruction size:"]
        #[inline(always)]
        pub fn set_isize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode. This field defines the address phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn admode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Address mode. This field defines the address phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_admode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Address double transfer rate. This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address double transfer rate. This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address size. This field defines address size."]
        #[inline(always)]
        pub const fn adsize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Address size. This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode. This field defines the alternate-byte phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn abmode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Alternate-byte mode. This field defines the alternate-byte phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double transfer rate. This bit sets the DTR mode for the alternate-bytes phase."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double transfer rate. This bit sets the DTR mode for the alternate-bytes phase."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size. This field defines alternate bytes size:"]
        #[inline(always)]
        pub const fn absize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Alternate bytes size. This field defines alternate bytes size:"]
        #[inline(always)]
        pub fn set_absize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode. This field defines the data phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn dmode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Data mode. This field defines the data phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "data double transfer rate. This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "data double transfer rate. This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DQS enable. This bit enables the data strobe management."]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DQS enable. This bit enables the data strobe management."]
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
    #[doc = "write instruction register"]
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
    #[doc = "wrap alternate bytes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpabr(pub u32);
    impl Wpabr {
        #[doc = "31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address"]
        #[inline(always)]
        pub const fn alternate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Alternate bytes Optional data to be sent to the external SPI device right after the address"]
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
    #[doc = "OCTOSPI wrap communication configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpccr(pub u32);
    impl Wpccr {
        #[doc = "Instruction mode. This field defines the instruction phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn imode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Instruction mode. This field defines the instruction phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_imode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Instruction double transfer rate. This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub const fn idtr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction double transfer rate. This bit sets the DTR mode for the instruction phase."]
        #[inline(always)]
        pub fn set_idtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Instruction size. This field defines instruction size."]
        #[inline(always)]
        pub const fn isize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Instruction size. This field defines instruction size."]
        #[inline(always)]
        pub fn set_isize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Address mode. This field defines the address phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn admode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Address mode. This field defines the address phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_admode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Address double transfer rate. This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub const fn addtr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Address double transfer rate. This bit sets the DTR mode for the address phase."]
        #[inline(always)]
        pub fn set_addtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Address size. This field defines address size."]
        #[inline(always)]
        pub const fn adsize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Address size. This field defines address size."]
        #[inline(always)]
        pub fn set_adsize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Alternate-byte mode. This field defines the alternate byte phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn abmode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Alternate-byte mode. This field defines the alternate byte phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_abmode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Alternate bytes double transfer rate. This bit sets the DTR mode for the alternate bytes phase."]
        #[inline(always)]
        pub const fn abdtr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate bytes double transfer rate. This bit sets the DTR mode for the alternate bytes phase."]
        #[inline(always)]
        pub fn set_abdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Alternate bytes size. This bit defines alternate bytes size."]
        #[inline(always)]
        pub const fn absize(&self) -> super::vals::SizeInBits {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::SizeInBits::from_bits(val as u8)
        }
        #[doc = "Alternate bytes size. This bit defines alternate bytes size."]
        #[inline(always)]
        pub fn set_absize(&mut self, val: super::vals::SizeInBits) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Data mode. This field defines the data phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub const fn dmode(&self) -> super::vals::PhaseMode {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::PhaseMode::from_bits(val as u8)
        }
        #[doc = "Data mode. This field defines the data phase mode of operation. 101-111: Reserved"]
        #[inline(always)]
        pub fn set_dmode(&mut self, val: super::vals::PhaseMode) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Data double transfer rate. This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub const fn ddtr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Data double transfer rate. This bit sets the DTR mode for the data phase."]
        #[inline(always)]
        pub fn set_ddtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DQS enable. This bit enables the data strobe management."]
        #[inline(always)]
        pub const fn dqse(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DQS enable. This bit enables the data strobe management."]
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
    #[doc = "wrap instruction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpir(pub u32);
    impl Wpir {
        #[doc = "31: 0\\]: Instruction Instruction to be sent to the external SPI device"]
        #[inline(always)]
        pub const fn instruction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "31: 0\\]: Instruction Instruction to be sent to the external SPI device"]
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
    #[doc = "wrap timing configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wptcr(pub u32);
    impl Wptcr {
        #[doc = "Number of dummy cycles. This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of dummy cycles. This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
        #[inline(always)]
        pub fn set_dcyc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay hold quarter cycle. Add a quarter cycle delay on the outputs in DTR communication to match hold requirement."]
        #[inline(always)]
        pub const fn dhqc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Delay hold quarter cycle. Add a quarter cycle delay on the outputs in DTR communication to match hold requirement."]
        #[inline(always)]
        pub fn set_dhqc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR = 1)."]
        #[inline(always)]
        pub const fn sshift(&self) -> super::vals::SampleShift {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::SampleShift::from_bits(val as u8)
        }
        #[doc = "Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode (when DDTR = 1)."]
        #[inline(always)]
        pub fn set_sshift(&mut self, val: super::vals::SampleShift) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
                "Wptcr {{ dcyc: {=u8:?}, dhqc: {=bool:?}, sshift: {:?} }}",
                self.dcyc(),
                self.dhqc(),
                self.sshift()
            )
        }
    }
    #[doc = "write timing configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtcr(pub u32);
    impl Wtcr {
        #[doc = "Number of dummy cycles. This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
        #[inline(always)]
        pub const fn dcyc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of dummy cycles. This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
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
    pub enum FlashSelect {
        #[doc = "FLASH 1 selected (data exchanged over IO\\[3:0\\])"]
        FLASH_ONE = 0x0,
        #[doc = "FLASH 2 selected (data exchanged over IO\\[7:4\\])"]
        FLASH_TWO = 0x01,
    }
    impl FlashSelect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FlashSelect {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FlashSelect {
        #[inline(always)]
        fn from(val: u8) -> FlashSelect {
            FlashSelect::from_bits(val)
        }
    }
    impl From<FlashSelect> for u8 {
        #[inline(always)]
        fn from(val: FlashSelect) -> u8 {
            FlashSelect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FunctionalMode {
        #[doc = "Indirect-write mode"]
        INDIRECT_WRITE = 0x0,
        #[doc = "Indirect-read mode"]
        INDIRECT_READ = 0x01,
        #[doc = "Automatic status-polling mode"]
        AUTO_STATUS_POLLING = 0x02,
        #[doc = "Memory-mapped mode"]
        MEMORY_MAPPED = 0x03,
    }
    impl FunctionalMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FunctionalMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FunctionalMode {
        #[inline(always)]
        fn from(val: u8) -> FunctionalMode {
            FunctionalMode::from_bits(val)
        }
    }
    impl From<FunctionalMode> for u8 {
        #[inline(always)]
        fn from(val: FunctionalMode) -> u8 {
            FunctionalMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LatencyMode {
        #[doc = "Variable initial latency"]
        VARIABLE = 0x0,
        #[doc = "Fixed latency"]
        FIXED = 0x01,
    }
    impl LatencyMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LatencyMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LatencyMode {
        #[inline(always)]
        fn from(val: u8) -> LatencyMode {
            LatencyMode::from_bits(val)
        }
    }
    impl From<LatencyMode> for u8 {
        #[inline(always)]
        fn from(val: LatencyMode) -> u8 {
            LatencyMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MatchMode {
        #[doc = "AND-match mode, SMF is set if all the unmasked bits received from the device match the corresponding bits in the match register."]
        MATCH_AND = 0x0,
        #[doc = "OR-match mode, SMF is set if any of the unmasked bits received from the device matches its corresponding bit in the match register."]
        MATCH_OR = 0x01,
    }
    impl MatchMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MatchMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MatchMode {
        #[inline(always)]
        fn from(val: u8) -> MatchMode {
            MatchMode::from_bits(val)
        }
    }
    impl From<MatchMode> for u8 {
        #[inline(always)]
        fn from(val: MatchMode) -> u8 {
            MatchMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemType {
        #[doc = "Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes."]
        MICRON = 0x0,
        #[doc = "Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes."]
        MACRONIX = 0x01,
        #[doc = "Standard mode"]
        B_STANDARD = 0x02,
        #[doc = "Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping."]
        MACRONIX_RAM = 0x03,
        #[doc = "HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected."]
        HYPER_BUS_MEMORY = 0x04,
        #[doc = "HyperBus register mode, addressing register space. The memory-mapped accesses in. this mode must be non-cacheable, or Indirect read/write modes must be used."]
        HYPER_BUS_REGISTER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl MemType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemType {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemType {
        #[inline(always)]
        fn from(val: u8) -> MemType {
            MemType::from_bits(val)
        }
    }
    impl From<MemType> for u8 {
        #[inline(always)]
        fn from(val: MemType) -> u8 {
            MemType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PhaseMode {
        #[doc = "No alternate bytes"]
        NONE = 0x0,
        #[doc = "Alternate bytes on a single line"]
        ONE_LINE = 0x01,
        #[doc = "Alternate bytes on two lines"]
        TWO_LINES = 0x02,
        #[doc = "Alternate bytes on four lines"]
        FOUR_LINES = 0x03,
        #[doc = "Alternate bytes on eight lines"]
        EIGHT_LINES = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl PhaseMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PhaseMode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PhaseMode {
        #[inline(always)]
        fn from(val: u8) -> PhaseMode {
            PhaseMode::from_bits(val)
        }
    }
    impl From<PhaseMode> for u8 {
        #[inline(always)]
        fn from(val: PhaseMode) -> u8 {
            PhaseMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SampleShift {
        #[doc = "No shift"]
        NONE = 0x0,
        #[doc = "1/2 cycle shift"]
        HALF_CYCLE = 0x01,
    }
    impl SampleShift {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleShift {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleShift {
        #[inline(always)]
        fn from(val: u8) -> SampleShift {
            SampleShift::from_bits(val)
        }
    }
    impl From<SampleShift> for u8 {
        #[inline(always)]
        fn from(val: SampleShift) -> u8 {
            SampleShift::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SizeInBits {
        #[doc = "8-bit alternate bytes"]
        _8BIT = 0x0,
        #[doc = "16-bit alternate bytes"]
        _16BIT = 0x01,
        #[doc = "24-bit alternate bytes"]
        _24BIT = 0x02,
        #[doc = "32-bit alternate bytes"]
        _32BIT = 0x03,
    }
    impl SizeInBits {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SizeInBits {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SizeInBits {
        #[inline(always)]
        fn from(val: u8) -> SizeInBits {
            SizeInBits::from_bits(val)
        }
    }
    impl From<SizeInBits> for u8 {
        #[inline(always)]
        fn from(val: SizeInBits) -> u8 {
            SizeInBits::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Threshold {
        #[doc = "FTF is set if there are one or more free bytes available to be written to in the FIFO in Indirect-write mode, or if there are one or more valid bytes can be read from the FIFO in Indirect-read mode."]
        NEED_ONE_BYTE = 0x0,
        #[doc = "FTF is set if there are two or more free bytes available to be written to in the FIFO in Indirect‑write mode, or if there are two or more valid bytes can be read from the FIFO in Indirect-read mode."]
        NEED_TWO_BYTES = 0x01,
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
        #[doc = "FTF is set if there are 32 free bytes available to be written to in the FIFO in Indirect-write mode, or if there are 32 valid bytes can be read from the FIFO in Indirect-read mode."]
        NEED_THIRTY_TWO_BYTES = 0x1f,
    }
    impl Threshold {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Threshold {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Threshold {
        #[inline(always)]
        fn from(val: u8) -> Threshold {
            Threshold::from_bits(val)
        }
    }
    impl From<Threshold> for u8 {
        #[inline(always)]
        fn from(val: Threshold) -> u8 {
            Threshold::to_bits(val)
        }
    }
}
