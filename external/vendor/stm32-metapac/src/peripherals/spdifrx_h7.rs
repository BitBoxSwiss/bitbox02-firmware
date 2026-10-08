#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Receiver Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdifrx {
    ptr: *mut u8,
}
unsafe impl Send for Spdifrx {}
unsafe impl Sync for Spdifrx {}
impl Spdifrx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt Flag Clear register"]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Data input register"]
    #[inline(always)]
    pub const fn fmt0_dr(self) -> crate::common::Reg<regs::Fmt0Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Data input register"]
    #[inline(always)]
    pub const fn fmt1_dr(self) -> crate::common::Reg<regs::Fmt1Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Data input register"]
    #[inline(always)]
    pub const fn fmt2_dr(self) -> crate::common::Reg<regs::Fmt2Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Channel Status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Debug Information register"]
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Peripheral Block Enable"]
        #[inline(always)]
        pub const fn spdifen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Peripheral Block Enable"]
        #[inline(always)]
        pub fn set_spdifen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Receiver DMA ENable for data flow"]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver DMA ENable for data flow"]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "STerEO Mode"]
        #[inline(always)]
        pub const fn rxsteo(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "STerEO Mode"]
        #[inline(always)]
        pub fn set_rxsteo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RX Data format"]
        #[inline(always)]
        pub const fn drfmt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RX Data format"]
        #[inline(always)]
        pub fn set_drfmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Mask Parity error bit"]
        #[inline(always)]
        pub const fn pmsk(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mask Parity error bit"]
        #[inline(always)]
        pub fn set_pmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Mask of Validity bit"]
        #[inline(always)]
        pub const fn vmsk(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Mask of Validity bit"]
        #[inline(always)]
        pub fn set_vmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Mask of channel status and user bits"]
        #[inline(always)]
        pub const fn cumsk(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Mask of channel status and user bits"]
        #[inline(always)]
        pub fn set_cumsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Mask of Preamble Type bits"]
        #[inline(always)]
        pub const fn ptmsk(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Mask of Preamble Type bits"]
        #[inline(always)]
        pub fn set_ptmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Control Buffer DMA ENable for control flow"]
        #[inline(always)]
        pub const fn cbdmaen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Control Buffer DMA ENable for control flow"]
        #[inline(always)]
        pub fn set_cbdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel Selection"]
        #[inline(always)]
        pub const fn chsel(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Selection"]
        #[inline(always)]
        pub fn set_chsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Maximum allowed re-tries during synchronization phase"]
        #[inline(always)]
        pub const fn nbtr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Maximum allowed re-tries during synchronization phase"]
        #[inline(always)]
        pub fn set_nbtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Wait For Activity"]
        #[inline(always)]
        pub const fn wfa(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wait For Activity"]
        #[inline(always)]
        pub fn set_wfa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "input selection"]
        #[inline(always)]
        pub const fn insel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "input selection"]
        #[inline(always)]
        pub fn set_insel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Symbol Clock Enable"]
        #[inline(always)]
        pub const fn cksen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Symbol Clock Enable"]
        #[inline(always)]
        pub fn set_cksen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Backup Symbol Clock Enable"]
        #[inline(always)]
        pub const fn cksbkpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Backup Symbol Clock Enable"]
        #[inline(always)]
        pub fn set_cksbkpen(&mut self, val: bool) {
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
                .field("spdifen", &self.spdifen())
                .field("rxdmaen", &self.rxdmaen())
                .field("rxsteo", &self.rxsteo())
                .field("drfmt", &self.drfmt())
                .field("pmsk", &self.pmsk())
                .field("vmsk", &self.vmsk())
                .field("cumsk", &self.cumsk())
                .field("ptmsk", &self.ptmsk())
                .field("cbdmaen", &self.cbdmaen())
                .field("chsel", &self.chsel())
                .field("nbtr", &self.nbtr())
                .field("wfa", &self.wfa())
                .field("insel", &self.insel())
                .field("cksen", &self.cksen())
                .field("cksbkpen", &self.cksbkpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ spdifen: {=u8:?}, rxdmaen: {=bool:?}, rxsteo: {=bool:?}, drfmt: {=u8:?}, pmsk: {=bool:?}, vmsk: {=bool:?}, cumsk: {=bool:?}, ptmsk: {=bool:?}, cbdmaen: {=bool:?}, chsel: {=bool:?}, nbtr: {=u8:?}, wfa: {=bool:?}, insel: {=u8:?}, cksen: {=bool:?}, cksbkpen: {=bool:?} }}" , self . spdifen () , self . rxdmaen () , self . rxsteo () , self . drfmt () , self . pmsk () , self . vmsk () , self . cumsk () , self . ptmsk () , self . cbdmaen () , self . chsel () , self . nbtr () , self . wfa () , self . insel () , self . cksen () , self . cksbkpen ())
        }
    }
    #[doc = "Channel Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "User data information"]
        #[inline(always)]
        pub const fn usr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "User data information"]
        #[inline(always)]
        pub fn set_usr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Channel A status information"]
        #[inline(always)]
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Channel A status information"]
        #[inline(always)]
        pub fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Start Of Block"]
        #[inline(always)]
        pub const fn sob(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Start Of Block"]
        #[inline(always)]
        pub fn set_sob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("usr", &self.usr())
                .field("cs", &self.cs())
                .field("sob", &self.sob())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ usr: {=u16:?}, cs: {=u8:?}, sob: {=bool:?} }}",
                self.usr(),
                self.cs(),
                self.sob()
            )
        }
    }
    #[doc = "Debug Information register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dir(pub u32);
    impl Dir {
        #[doc = "Threshold HIGH"]
        #[inline(always)]
        pub const fn thi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Threshold HIGH"]
        #[inline(always)]
        pub fn set_thi(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Threshold LOW"]
        #[inline(always)]
        pub const fn tlo(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Threshold LOW"]
        #[inline(always)]
        pub fn set_tlo(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Dir {
        #[inline(always)]
        fn default() -> Dir {
            Dir(0)
        }
    }
    impl core::fmt::Debug for Dir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dir")
                .field("thi", &self.thi())
                .field("tlo", &self.tlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dir {{ thi: {=u16:?}, tlo: {=u16:?} }}", self.thi(), self.tlo())
        }
    }
    #[doc = "FMT0 data input register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmt0Dr(pub u32);
    impl Fmt0Dr {
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Validity bit"]
        #[inline(always)]
        pub const fn v(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Validity bit"]
        #[inline(always)]
        pub fn set_v(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "User bit"]
        #[inline(always)]
        pub const fn u(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "User bit"]
        #[inline(always)]
        pub fn set_u(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Channel Status bit"]
        #[inline(always)]
        pub const fn c(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Status bit"]
        #[inline(always)]
        pub fn set_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Preamble Type"]
        #[inline(always)]
        pub const fn pt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Preamble Type"]
        #[inline(always)]
        pub fn set_pt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Fmt0Dr {
        #[inline(always)]
        fn default() -> Fmt0Dr {
            Fmt0Dr(0)
        }
    }
    impl core::fmt::Debug for Fmt0Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmt0Dr")
                .field("dr", &self.dr())
                .field("pe", &self.pe())
                .field("v", &self.v())
                .field("u", &self.u())
                .field("c", &self.c())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fmt0Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fmt0Dr {{ dr: {=u32:?}, pe: {=bool:?}, v: {=bool:?}, u: {=bool:?}, c: {=bool:?}, pt: {=u8:?} }}",
                self.dr(),
                self.pe(),
                self.v(),
                self.u(),
                self.c(),
                self.pt()
            )
        }
    }
    #[doc = "FMT1 data input register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmt1Dr(pub u32);
    impl Fmt1Dr {
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Validity bit"]
        #[inline(always)]
        pub const fn v(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Validity bit"]
        #[inline(always)]
        pub fn set_v(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "User bit"]
        #[inline(always)]
        pub const fn u(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "User bit"]
        #[inline(always)]
        pub fn set_u(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel Status bit"]
        #[inline(always)]
        pub const fn c(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Status bit"]
        #[inline(always)]
        pub fn set_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Preamble Type"]
        #[inline(always)]
        pub const fn pt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Preamble Type"]
        #[inline(always)]
        pub fn set_pt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Parity Error bit"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Fmt1Dr {
        #[inline(always)]
        fn default() -> Fmt1Dr {
            Fmt1Dr(0)
        }
    }
    impl core::fmt::Debug for Fmt1Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmt1Dr")
                .field("pe", &self.pe())
                .field("v", &self.v())
                .field("u", &self.u())
                .field("c", &self.c())
                .field("pt", &self.pt())
                .field("dr", &self.dr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fmt1Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fmt1Dr {{ pe: {=bool:?}, v: {=bool:?}, u: {=bool:?}, c: {=bool:?}, pt: {=u8:?}, dr: {=u32:?} }}",
                self.pe(),
                self.v(),
                self.u(),
                self.c(),
                self.pt(),
                self.dr()
            )
        }
    }
    #[doc = "FMT2 data input register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmt2Dr(pub u32);
    impl Fmt2Dr {
        #[doc = "Channel A data value"]
        #[inline(always)]
        pub const fn drnl1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel A data value"]
        #[inline(always)]
        pub fn set_drnl1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Channel B data value"]
        #[inline(always)]
        pub const fn drnl2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel B data value"]
        #[inline(always)]
        pub fn set_drnl2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Fmt2Dr {
        #[inline(always)]
        fn default() -> Fmt2Dr {
            Fmt2Dr(0)
        }
    }
    impl core::fmt::Debug for Fmt2Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmt2Dr")
                .field("drnl1", &self.drnl1())
                .field("drnl2", &self.drnl2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fmt2Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fmt2Dr {{ drnl1: {=u16:?}, drnl2: {=u16:?} }}",
                self.drnl1(),
                self.drnl2()
            )
        }
    }
    #[doc = "Interrupt Flag Clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifcr(pub u32);
    impl Ifcr {
        #[doc = "Clears the Parity error flag"]
        #[inline(always)]
        pub const fn perrcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clears the Parity error flag"]
        #[inline(always)]
        pub fn set_perrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clears the Overrun error flag"]
        #[inline(always)]
        pub const fn ovrcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clears the Overrun error flag"]
        #[inline(always)]
        pub fn set_ovrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clears the Synchronization Block Detected flag"]
        #[inline(always)]
        pub const fn sbdcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clears the Synchronization Block Detected flag"]
        #[inline(always)]
        pub fn set_sbdcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clears the Synchronization Done flag"]
        #[inline(always)]
        pub const fn syncdcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clears the Synchronization Done flag"]
        #[inline(always)]
        pub fn set_syncdcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ifcr {
        #[inline(always)]
        fn default() -> Ifcr {
            Ifcr(0)
        }
    }
    impl core::fmt::Debug for Ifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifcr")
                .field("perrcf", &self.perrcf())
                .field("ovrcf", &self.ovrcf())
                .field("sbdcf", &self.sbdcf())
                .field("syncdcf", &self.syncdcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ifcr {{ perrcf: {=bool:?}, ovrcf: {=bool:?}, sbdcf: {=bool:?}, syncdcf: {=bool:?} }}",
                self.perrcf(),
                self.ovrcf(),
                self.sbdcf(),
                self.syncdcf()
            )
        }
    }
    #[doc = "Interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr(pub u32);
    impl Imr {
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Control Buffer Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn csrneie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Control Buffer Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_csrneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error interrupt enable"]
        #[inline(always)]
        pub const fn perrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error interrupt enable"]
        #[inline(always)]
        pub fn set_perrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error Interrupt Enable"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error Interrupt Enable"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Synchronization Block Detected Interrupt Enable"]
        #[inline(always)]
        pub const fn sblkie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Block Detected Interrupt Enable"]
        #[inline(always)]
        pub fn set_sblkie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Synchronization Done"]
        #[inline(always)]
        pub const fn syncdie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Done"]
        #[inline(always)]
        pub fn set_syncdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Serial Interface Error Interrupt Enable"]
        #[inline(always)]
        pub const fn ifeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Serial Interface Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_ifeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("rxneie", &self.rxneie())
                .field("csrneie", &self.csrneie())
                .field("perrie", &self.perrie())
                .field("ovrie", &self.ovrie())
                .field("sblkie", &self.sblkie())
                .field("syncdie", &self.syncdie())
                .field("ifeie", &self.ifeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Imr {{ rxneie: {=bool:?}, csrneie: {=bool:?}, perrie: {=bool:?}, ovrie: {=bool:?}, sblkie: {=bool:?}, syncdie: {=bool:?}, ifeie: {=bool:?} }}" , self . rxneie () , self . csrneie () , self . perrie () , self . ovrie () , self . sblkie () , self . syncdie () , self . ifeie ())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Control Buffer register is not empty"]
        #[inline(always)]
        pub const fn csrne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Control Buffer register is not empty"]
        #[inline(always)]
        pub fn set_csrne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Synchronization Block Detected"]
        #[inline(always)]
        pub const fn sbd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Block Detected"]
        #[inline(always)]
        pub fn set_sbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Synchronization Done"]
        #[inline(always)]
        pub const fn syncd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Done"]
        #[inline(always)]
        pub fn set_syncd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub const fn ferr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub fn set_ferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Synchronization error"]
        #[inline(always)]
        pub const fn serr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error"]
        #[inline(always)]
        pub fn set_serr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Time-out error"]
        #[inline(always)]
        pub const fn terr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Time-out error"]
        #[inline(always)]
        pub fn set_terr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Duration of 5 symbols counted with SPDIF_CLK"]
        #[inline(always)]
        pub const fn width(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Duration of 5 symbols counted with SPDIF_CLK"]
        #[inline(always)]
        pub fn set_width(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
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
                .field("rxne", &self.rxne())
                .field("csrne", &self.csrne())
                .field("perr", &self.perr())
                .field("ovr", &self.ovr())
                .field("sbd", &self.sbd())
                .field("syncd", &self.syncd())
                .field("ferr", &self.ferr())
                .field("serr", &self.serr())
                .field("terr", &self.terr())
                .field("width", &self.width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ rxne: {=bool:?}, csrne: {=bool:?}, perr: {=bool:?}, ovr: {=bool:?}, sbd: {=bool:?}, syncd: {=bool:?}, ferr: {=bool:?}, serr: {=bool:?}, terr: {=bool:?}, width: {=u16:?} }}" , self . rxne () , self . csrne () , self . perr () , self . ovr () , self . sbd () , self . syncd () , self . ferr () , self . serr () , self . terr () , self . width ())
        }
    }
}
