#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital camera interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcmi {
    ptr: *mut u8,
}
unsafe impl Send for Dcmi {}
unsafe impl Sync for Dcmi {}
impl Dcmi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "embedded synchronization code register"]
    #[inline(always)]
    pub const fn escr(self) -> crate::common::Reg<regs::Escr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "embedded synchronization unmask register"]
    #[inline(always)]
    pub const fn esur(self) -> crate::common::Reg<regs::Esur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "crop window start"]
    #[inline(always)]
    pub const fn cwstrt(self) -> crate::common::Reg<regs::Cwstrt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "crop window size"]
    #[inline(always)]
    pub const fn cwsize(self) -> crate::common::Reg<regs::Cwsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Capture enable"]
        #[inline(always)]
        pub const fn capture(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture enable"]
        #[inline(always)]
        pub fn set_capture(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture mode"]
        #[inline(always)]
        pub const fn cm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Crop feature"]
        #[inline(always)]
        pub const fn crop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Crop feature"]
        #[inline(always)]
        pub fn set_crop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "JPEG format"]
        #[inline(always)]
        pub const fn jpeg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG format"]
        #[inline(always)]
        pub fn set_jpeg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Embedded synchronization select"]
        #[inline(always)]
        pub const fn ess(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Embedded synchronization select"]
        #[inline(always)]
        pub fn set_ess(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pixel clock polarity"]
        #[inline(always)]
        pub const fn pckpol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel clock polarity"]
        #[inline(always)]
        pub fn set_pckpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Horizontal synchronization polarity"]
        #[inline(always)]
        pub const fn hspol(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal synchronization polarity"]
        #[inline(always)]
        pub fn set_hspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Vertical synchronization polarity"]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization polarity"]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame capture rate control"]
        #[inline(always)]
        pub const fn fcrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control"]
        #[inline(always)]
        pub fn set_fcrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Extended data mode"]
        #[inline(always)]
        pub const fn edm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Extended data mode"]
        #[inline(always)]
        pub fn set_edm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "DCMI enable"]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI enable"]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("capture", &self.capture())
                .field("cm", &self.cm())
                .field("crop", &self.crop())
                .field("jpeg", &self.jpeg())
                .field("ess", &self.ess())
                .field("pckpol", &self.pckpol())
                .field("hspol", &self.hspol())
                .field("vspol", &self.vspol())
                .field("fcrc", &self.fcrc())
                .field("edm", &self.edm())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ capture: {=bool:?}, cm: {=bool:?}, crop: {=bool:?}, jpeg: {=bool:?}, ess: {=bool:?}, pckpol: {=bool:?}, hspol: {=bool:?}, vspol: {=bool:?}, fcrc: {=u8:?}, edm: {=u8:?}, enable: {=bool:?} }}" , self . capture () , self . cm () , self . crop () , self . jpeg () , self . ess () , self . pckpol () , self . hspol () , self . vspol () , self . fcrc () , self . edm () , self . enable ())
        }
    }
    #[doc = "crop window size"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cwsize(pub u32);
    impl Cwsize {
        #[doc = "Capture count"]
        #[inline(always)]
        pub const fn capcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Capture count"]
        #[inline(always)]
        pub fn set_capcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "Vertical line count"]
        #[inline(always)]
        pub const fn vline(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical line count"]
        #[inline(always)]
        pub fn set_vline(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Cwsize {
        #[inline(always)]
        fn default() -> Cwsize {
            Cwsize(0)
        }
    }
    impl core::fmt::Debug for Cwsize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cwsize")
                .field("capcnt", &self.capcnt())
                .field("vline", &self.vline())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cwsize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cwsize {{ capcnt: {=u16:?}, vline: {=u16:?} }}",
                self.capcnt(),
                self.vline()
            )
        }
    }
    #[doc = "crop window start"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cwstrt(pub u32);
    impl Cwstrt {
        #[doc = "Horizontal offset count"]
        #[inline(always)]
        pub const fn hoffcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Horizontal offset count"]
        #[inline(always)]
        pub fn set_hoffcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "Vertical start line count"]
        #[inline(always)]
        pub const fn vst(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Vertical start line count"]
        #[inline(always)]
        pub fn set_vst(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Cwstrt {
        #[inline(always)]
        fn default() -> Cwstrt {
            Cwstrt(0)
        }
    }
    impl core::fmt::Debug for Cwstrt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cwstrt")
                .field("hoffcnt", &self.hoffcnt())
                .field("vst", &self.vst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cwstrt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cwstrt {{ hoffcnt: {=u16:?}, vst: {=u16:?} }}",
                self.hoffcnt(),
                self.vst()
            )
        }
    }
    #[doc = "data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data byte 0"]
        #[inline(always)]
        pub const fn byte0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 0"]
        #[inline(always)]
        pub fn set_byte0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data byte 1"]
        #[inline(always)]
        pub const fn byte1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 1"]
        #[inline(always)]
        pub fn set_byte1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data byte 2"]
        #[inline(always)]
        pub const fn byte2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 2"]
        #[inline(always)]
        pub fn set_byte2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data byte 3"]
        #[inline(always)]
        pub const fn byte3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data byte 3"]
        #[inline(always)]
        pub fn set_byte3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
                .field("byte0", &self.byte0())
                .field("byte1", &self.byte1())
                .field("byte2", &self.byte2())
                .field("byte3", &self.byte3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dr {{ byte0: {=u8:?}, byte1: {=u8:?}, byte2: {=u8:?}, byte3: {=u8:?} }}",
                self.byte0(),
                self.byte1(),
                self.byte2(),
                self.byte3()
            )
        }
    }
    #[doc = "embedded synchronization code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Escr(pub u32);
    impl Escr {
        #[doc = "Frame start delimiter code"]
        #[inline(always)]
        pub const fn fsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame start delimiter code"]
        #[inline(always)]
        pub fn set_fsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Line start delimiter code"]
        #[inline(always)]
        pub const fn lsc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Line start delimiter code"]
        #[inline(always)]
        pub fn set_lsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Line end delimiter code"]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Line end delimiter code"]
        #[inline(always)]
        pub fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Frame end delimiter code"]
        #[inline(always)]
        pub const fn fec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Frame end delimiter code"]
        #[inline(always)]
        pub fn set_fec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Escr {
        #[inline(always)]
        fn default() -> Escr {
            Escr(0)
        }
    }
    impl core::fmt::Debug for Escr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Escr")
                .field("fsc", &self.fsc())
                .field("lsc", &self.lsc())
                .field("lec", &self.lec())
                .field("fec", &self.fec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Escr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Escr {{ fsc: {=u8:?}, lsc: {=u8:?}, lec: {=u8:?}, fec: {=u8:?} }}",
                self.fsc(),
                self.lsc(),
                self.lec(),
                self.fec()
            )
        }
    }
    #[doc = "embedded synchronization unmask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esur(pub u32);
    impl Esur {
        #[doc = "Frame start delimiter unmask"]
        #[inline(always)]
        pub const fn fsu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame start delimiter unmask"]
        #[inline(always)]
        pub fn set_fsu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Line start delimiter unmask"]
        #[inline(always)]
        pub const fn lsu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Line start delimiter unmask"]
        #[inline(always)]
        pub fn set_lsu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Line end delimiter unmask"]
        #[inline(always)]
        pub const fn leu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Line end delimiter unmask"]
        #[inline(always)]
        pub fn set_leu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Frame end delimiter unmask"]
        #[inline(always)]
        pub const fn feu(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Frame end delimiter unmask"]
        #[inline(always)]
        pub fn set_feu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Esur {
        #[inline(always)]
        fn default() -> Esur {
            Esur(0)
        }
    }
    impl core::fmt::Debug for Esur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esur")
                .field("fsu", &self.fsu())
                .field("lsu", &self.lsu())
                .field("leu", &self.leu())
                .field("feu", &self.feu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Esur {{ fsu: {=u8:?}, lsu: {=u8:?}, leu: {=u8:?}, feu: {=u8:?} }}",
                self.fsu(),
                self.lsu(),
                self.leu(),
                self.feu()
            )
        }
    }
    #[doc = "interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Capture complete interrupt status clear"]
        #[inline(always)]
        pub const fn frame_isc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture complete interrupt status clear"]
        #[inline(always)]
        pub fn set_frame_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun interrupt status clear"]
        #[inline(always)]
        pub const fn ovr_isc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear"]
        #[inline(always)]
        pub fn set_ovr_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronization error interrupt status clear"]
        #[inline(always)]
        pub const fn err_isc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error interrupt status clear"]
        #[inline(always)]
        pub fn set_err_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Vertical synch interrupt status clear"]
        #[inline(always)]
        pub const fn vsync_isc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synch interrupt status clear"]
        #[inline(always)]
        pub fn set_vsync_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "line interrupt status clear"]
        #[inline(always)]
        pub const fn line_isc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "line interrupt status clear"]
        #[inline(always)]
        pub fn set_line_isc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("frame_isc", &self.frame_isc())
                .field("ovr_isc", &self.ovr_isc())
                .field("err_isc", &self.err_isc())
                .field("vsync_isc", &self.vsync_isc())
                .field("line_isc", &self.line_isc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ frame_isc: {=bool:?}, ovr_isc: {=bool:?}, err_isc: {=bool:?}, vsync_isc: {=bool:?}, line_isc: {=bool:?} }}" , self . frame_isc () , self . ovr_isc () , self . err_isc () , self . vsync_isc () , self . line_isc ())
        }
    }
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Capture complete interrupt enable"]
        #[inline(always)]
        pub const fn frame_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture complete interrupt enable"]
        #[inline(always)]
        pub fn set_frame_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub const fn ovr_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub fn set_ovr_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronization error interrupt enable"]
        #[inline(always)]
        pub const fn err_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error interrupt enable"]
        #[inline(always)]
        pub fn set_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VSYNC interrupt enable"]
        #[inline(always)]
        pub const fn vsync_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC interrupt enable"]
        #[inline(always)]
        pub fn set_vsync_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Line interrupt enable"]
        #[inline(always)]
        pub const fn line_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Line interrupt enable"]
        #[inline(always)]
        pub fn set_line_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            f.debug_struct("Ier")
                .field("frame_ie", &self.frame_ie())
                .field("ovr_ie", &self.ovr_ie())
                .field("err_ie", &self.err_ie())
                .field("vsync_ie", &self.vsync_ie())
                .field("line_ie", &self.line_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ frame_ie: {=bool:?}, ovr_ie: {=bool:?}, err_ie: {=bool:?}, vsync_ie: {=bool:?}, line_ie: {=bool:?} }}" , self . frame_ie () , self . ovr_ie () , self . err_ie () , self . vsync_ie () , self . line_ie ())
        }
    }
    #[doc = "masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mis(pub u32);
    impl Mis {
        #[doc = "Capture complete masked interrupt status"]
        #[inline(always)]
        pub const fn frame_mis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture complete masked interrupt status"]
        #[inline(always)]
        pub fn set_frame_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun masked interrupt status"]
        #[inline(always)]
        pub const fn ovr_mis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun masked interrupt status"]
        #[inline(always)]
        pub fn set_ovr_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronization error masked interrupt status"]
        #[inline(always)]
        pub const fn err_mis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error masked interrupt status"]
        #[inline(always)]
        pub fn set_err_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VSYNC masked interrupt status"]
        #[inline(always)]
        pub const fn vsync_mis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC masked interrupt status"]
        #[inline(always)]
        pub fn set_vsync_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Line masked interrupt status"]
        #[inline(always)]
        pub const fn line_mis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Line masked interrupt status"]
        #[inline(always)]
        pub fn set_line_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            f.debug_struct("Mis")
                .field("frame_mis", &self.frame_mis())
                .field("ovr_mis", &self.ovr_mis())
                .field("err_mis", &self.err_mis())
                .field("vsync_mis", &self.vsync_mis())
                .field("line_mis", &self.line_mis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mis {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mis {{ frame_mis: {=bool:?}, ovr_mis: {=bool:?}, err_mis: {=bool:?}, vsync_mis: {=bool:?}, line_mis: {=bool:?} }}" , self . frame_mis () , self . ovr_mis () , self . err_mis () , self . vsync_mis () , self . line_mis ())
        }
    }
    #[doc = "raw interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ris(pub u32);
    impl Ris {
        #[doc = "Capture complete raw interrupt status"]
        #[inline(always)]
        pub const fn frame_ris(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture complete raw interrupt status"]
        #[inline(always)]
        pub fn set_frame_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun raw interrupt status"]
        #[inline(always)]
        pub const fn ovr_ris(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status"]
        #[inline(always)]
        pub fn set_ovr_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronization error raw interrupt status"]
        #[inline(always)]
        pub const fn err_ris(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error raw interrupt status"]
        #[inline(always)]
        pub fn set_err_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VSYNC raw interrupt status"]
        #[inline(always)]
        pub const fn vsync_ris(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status"]
        #[inline(always)]
        pub fn set_vsync_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Line raw interrupt status"]
        #[inline(always)]
        pub const fn line_ris(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Line raw interrupt status"]
        #[inline(always)]
        pub fn set_line_ris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            f.debug_struct("Ris")
                .field("frame_ris", &self.frame_ris())
                .field("ovr_ris", &self.ovr_ris())
                .field("err_ris", &self.err_ris())
                .field("vsync_ris", &self.vsync_ris())
                .field("line_ris", &self.line_ris())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ris {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ris {{ frame_ris: {=bool:?}, ovr_ris: {=bool:?}, err_ris: {=bool:?}, vsync_ris: {=bool:?}, line_ris: {=bool:?} }}" , self . frame_ris () , self . ovr_ris () , self . err_ris () , self . vsync_ris () , self . line_ris ())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "HSYNC"]
        #[inline(always)]
        pub const fn hsync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSYNC"]
        #[inline(always)]
        pub fn set_hsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VSYNC"]
        #[inline(always)]
        pub const fn vsync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC"]
        #[inline(always)]
        pub fn set_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO not empty"]
        #[inline(always)]
        pub const fn fne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO not empty"]
        #[inline(always)]
        pub fn set_fne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("hsync", &self.hsync())
                .field("vsync", &self.vsync())
                .field("fne", &self.fne())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ hsync: {=bool:?}, vsync: {=bool:?}, fne: {=bool:?} }}",
                self.hsync(),
                self.vsync(),
                self.fne()
            )
        }
    }
}
