#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMA2D"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma2d {
    ptr: *mut u8,
}
unsafe impl Send for Dma2d {}
unsafe impl Sync for Dma2d {}
impl Dma2d {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA2D control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA2D interrupt status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA2D interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA2D foreground memory address register"]
    #[inline(always)]
    pub const fn fgmar(self) -> crate::common::Reg<regs::Fgmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DMA2D foreground offset register"]
    #[inline(always)]
    pub const fn fgor(self) -> crate::common::Reg<regs::Fgor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DMA2D background memory address register"]
    #[inline(always)]
    pub const fn bgmar(self) -> crate::common::Reg<regs::Bgmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DMA2D background offset register"]
    #[inline(always)]
    pub const fn bgor(self) -> crate::common::Reg<regs::Bgor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DMA2D foreground PFC (pixel format converter) control register"]
    #[inline(always)]
    pub const fn fgpfccr(self) -> crate::common::Reg<regs::Fgpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "DMA2D foreground color register"]
    #[inline(always)]
    pub const fn fgcolr(self) -> crate::common::Reg<regs::Fgcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DMA2D background PFC (pixel format converter) control register"]
    #[inline(always)]
    pub const fn bgpfccr(self) -> crate::common::Reg<regs::Bgpfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "DMA2D background color register"]
    #[inline(always)]
    pub const fn bgcolr(self) -> crate::common::Reg<regs::Bgcolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "DMA2D foreground CLUT memory address register"]
    #[inline(always)]
    pub const fn fgcmar(self) -> crate::common::Reg<regs::Fgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "DMA2D background CLUT memory address register"]
    #[inline(always)]
    pub const fn bgcmar(self) -> crate::common::Reg<regs::Bgcmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "DMA2D output PFC (pixel format converter) control register"]
    #[inline(always)]
    pub const fn opfccr(self) -> crate::common::Reg<regs::Opfccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "DMA2D output color register"]
    #[inline(always)]
    pub const fn ocolr(self) -> crate::common::Reg<regs::Ocolr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "DMA2D output memory address register"]
    #[inline(always)]
    pub const fn omar(self) -> crate::common::Reg<regs::Omar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "DMA2D output offset register"]
    #[inline(always)]
    pub const fn oor(self) -> crate::common::Reg<regs::Oor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DMA2D number of line register"]
    #[inline(always)]
    pub const fn nlr(self) -> crate::common::Reg<regs::Nlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DMA2D line watermark register"]
    #[inline(always)]
    pub const fn lwr(self) -> crate::common::Reg<regs::Lwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DMA2D AXI master timer configuration register"]
    #[inline(always)]
    pub const fn amtcr(self) -> crate::common::Reg<regs::Amtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA2D AXI master timer configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Amtcr(pub u32);
    impl Amtcr {
        #[doc = "Enable. Enables the dead time functionality."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable. Enables the dead time functionality."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Dead time. Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Dead time. Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Amtcr {
        #[inline(always)]
        fn default() -> Amtcr {
            Amtcr(0)
        }
    }
    impl core::fmt::Debug for Amtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Amtcr")
                .field("en", &self.en())
                .field("dt", &self.dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Amtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Amtcr {{ en: {=bool:?}, dt: {=u8:?} }}", self.en(), self.dt())
        }
    }
    #[doc = "DMA2D background CLUT memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgcmar(pub u32);
    impl Bgcmar {
        #[doc = "Memory address. Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory address. Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bgcmar {
        #[inline(always)]
        fn default() -> Bgcmar {
            Bgcmar(0)
        }
    }
    impl core::fmt::Debug for Bgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgcmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgcmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgcmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "DMA2D background color register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgcolr(pub u32);
    impl Bgcolr {
        #[doc = "Blue value. These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue value. These bits define the blue value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green value. These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green value. These bits define the green value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red value. These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red value. These bits define the red value for the A4 or A8 mode of the background. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bgcolr {
        #[inline(always)]
        fn default() -> Bgcolr {
            Bgcolr(0)
        }
    }
    impl core::fmt::Debug for Bgcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgcolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bgcolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red()
            )
        }
    }
    #[doc = "DMA2D background memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgmar(pub u32);
    impl Bgmar {
        #[doc = "Memory address. Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory address. Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bgmar {
        #[inline(always)]
        fn default() -> Bgmar {
            Bgmar(0)
        }
    }
    impl core::fmt::Debug for Bgmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "DMA2D background offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgor(pub u32);
    impl Bgor {
        #[doc = "Line offset used for the background image (expressed in pixels (default) or bytes as per LOM in CR). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line offset used for the background image (expressed in pixels (default) or bytes as per LOM in CR). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Bgor {
        #[inline(always)]
        fn default() -> Bgor {
            Bgor(0)
        }
    }
    impl core::fmt::Debug for Bgor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[doc = "DMA2D background PFC (pixel format converter) control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgpfccr(pub u32);
    impl Bgpfccr {
        #[doc = "Color mode. These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub const fn cm(&self) -> super::vals::BgpfccrCm {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::BgpfccrCm::from_bits(val as u8)
        }
        #[doc = "Color mode. These bits define the color format of the foreground image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: super::vals::BgpfccrCm) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "CLUT (color lookup table) color mode. These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn ccm(&self) -> super::vals::BgpfccrCcm {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::BgpfccrCcm::from_bits(val as u8)
        }
        #[doc = "CLUT (color lookup table) color mode. These bits define the color format of the CLUT. This register can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_ccm(&mut self, val: super::vals::BgpfccrCcm) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Start. This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::BgpfccrStart {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::BgpfccrStart::from_bits(val as u8)
        }
        #[doc = "Start. This bit is set to start the automatic loading of the CLUT. This bit is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in the DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already on going (data transfer or automatic BackGround CLUT transfer)."]
        #[inline(always)]
        pub fn set_start(&mut self, val: super::vals::BgpfccrStart) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CLUT size. These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
        #[inline(always)]
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT size. These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Alpha mode. These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub const fn am(&self) -> super::vals::BgpfccrAm {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::BgpfccrAm::from_bits(val as u8)
        }
        #[doc = "Alpha mode. These bits define which alpha channel value to be used for the background image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub fn set_am(&mut self, val: super::vals::BgpfccrAm) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn ai(&self) -> super::vals::BgpfccrAi {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::BgpfccrAi::from_bits(val as u8)
        }
        #[doc = "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_ai(&mut self, val: super::vals::BgpfccrAi) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn rbs(&self) -> super::vals::BgpfccrRbs {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::BgpfccrRbs::from_bits(val as u8)
        }
        #[doc = "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_rbs(&mut self, val: super::vals::BgpfccrRbs) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Alpha value. These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Alpha value. These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied with the original alpha value according to the alpha mode selected with bits AM\\[1: 0\\]. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Bgpfccr {
        #[inline(always)]
        fn default() -> Bgpfccr {
            Bgpfccr(0)
        }
    }
    impl core::fmt::Debug for Bgpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgpfccr")
                .field("cm", &self.cm())
                .field("ccm", &self.ccm())
                .field("start", &self.start())
                .field("cs", &self.cs())
                .field("am", &self.am())
                .field("ai", &self.ai())
                .field("rbs", &self.rbs())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bgpfccr {{ cm: {:?}, ccm: {:?}, start: {:?}, cs: {=u8:?}, am: {:?}, ai: {:?}, rbs: {:?}, alpha: {=u8:?} }}" , self . cm () , self . ccm () , self . start () , self . cs () , self . am () , self . ai () , self . rbs () , self . alpha ())
        }
    }
    #[doc = "DMA2D control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Start. This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::CrStart {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::CrStart::from_bits(val as u8)
        }
        #[doc = "Start. This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
        #[inline(always)]
        pub fn set_start(&mut self, val: super::vals::CrStart) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Suspend. This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend. This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Abort. This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
        #[inline(always)]
        pub const fn abort(&self) -> super::vals::Abort {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Abort::from_bits(val as u8)
        }
        #[doc = "Abort. This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
        #[inline(always)]
        pub fn set_abort(&mut self, val: super::vals::Abort) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Line offset mode. This bit configures how the line offset is expressed (in pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing."]
        #[inline(always)]
        pub const fn lom(&self) -> super::vals::Lom {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Lom::from_bits(val as u8)
        }
        #[doc = "Line offset mode. This bit configures how the line offset is expressed (in pixels or bytes) for the foreground, background and output. This bit is set and cleared by software. It can not be modified while a transfer is ongoing."]
        #[inline(always)]
        pub fn set_lom(&mut self, val: super::vals::Lom) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Transfer error interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transfer complete interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transfer watermark interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn twie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer watermark interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_twie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CLUT access error interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn caeie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT access error interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_caeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CLUT transfer complete interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn ctcie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT transfer complete interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_ctcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Configuration error interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn ceie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration error interrupt enable. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_ceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
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
                .field("start", &self.start())
                .field("susp", &self.susp())
                .field("abort", &self.abort())
                .field("lom", &self.lom())
                .field("teie", &self.teie())
                .field("tcie", &self.tcie())
                .field("twie", &self.twie())
                .field("caeie", &self.caeie())
                .field("ctcie", &self.ctcie())
                .field("ceie", &self.ceie())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ start: {:?}, susp: {=bool:?}, abort: {:?}, lom: {:?}, teie: {=bool:?}, tcie: {=bool:?}, twie: {=bool:?}, caeie: {=bool:?}, ctcie: {=bool:?}, ceie: {=bool:?}, mode: {:?} }}" , self . start () , self . susp () , self . abort () , self . lom () , self . teie () , self . tcie () , self . twie () , self . caeie () , self . ctcie () , self . ceie () , self . mode ())
        }
    }
    #[doc = "DMA2D foreground CLUT memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgcmar(pub u32);
    impl Fgcmar {
        #[doc = "Memory Address. Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory Address. Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fgcmar {
        #[inline(always)]
        fn default() -> Fgcmar {
            Fgcmar(0)
        }
    }
    impl core::fmt::Debug for Fgcmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgcmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgcmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fgcmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "DMA2D foreground color register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgcolr(pub u32);
    impl Fgcolr {
        #[doc = "Blue Value. These bits define the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue Value. These bits define the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green Value. These bits define the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green Value. These bits define the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red Value. These bits define the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red Value. These bits define the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Fgcolr {
        #[inline(always)]
        fn default() -> Fgcolr {
            Fgcolr(0)
        }
    }
    impl core::fmt::Debug for Fgcolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgcolr")
                .field("blue", &self.blue())
                .field("green", &self.green())
                .field("red", &self.red())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgcolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fgcolr {{ blue: {=u8:?}, green: {=u8:?}, red: {=u8:?} }}",
                self.blue(),
                self.green(),
                self.red()
            )
        }
    }
    #[doc = "DMA2D foreground memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgmar(pub u32);
    impl Fgmar {
        #[doc = "Memory address. Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory address. Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fgmar {
        #[inline(always)]
        fn default() -> Fgmar {
            Fgmar(0)
        }
    }
    impl core::fmt::Debug for Fgmar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgmar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgmar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fgmar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "DMA2D foreground offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgor(pub u32);
    impl Fgor {
        #[doc = "Line offset. Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line offset. Line offset used for the foreground expressed in pixel. This value is used to generate the address. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once a data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Fgor {
        #[inline(always)]
        fn default() -> Fgor {
            Fgor(0)
        }
    }
    impl core::fmt::Debug for Fgor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fgor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[doc = "DMA2D foreground PFC (pixel format converter) control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fgpfccr(pub u32);
    impl Fgpfccr {
        #[doc = "Color mode. These bits define the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub const fn cm(&self) -> super::vals::FgpfccrCm {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::FgpfccrCm::from_bits(val as u8)
        }
        #[doc = "Color mode. These bits define the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: super::vals::FgpfccrCm) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "CLUT color mode. This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn ccm(&self) -> super::vals::FgpfccrCcm {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::FgpfccrCcm::from_bits(val as u8)
        }
        #[doc = "CLUT color mode. This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_ccm(&mut self, val: super::vals::FgpfccrCcm) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Start. This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer)."]
        #[inline(always)]
        pub const fn start(&self) -> super::vals::FgpfccrStart {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::FgpfccrStart::from_bits(val as u8)
        }
        #[doc = "Start. This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer)."]
        #[inline(always)]
        pub fn set_start(&mut self, val: super::vals::FgpfccrStart) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CLUT size. These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
        #[inline(always)]
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT size. These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\\[7:0\\]
+ 1."]
        #[inline(always)]
        pub fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Alpha mode. These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless"]
        #[inline(always)]
        pub const fn am(&self) -> super::vals::FgpfccrAm {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::FgpfccrAm::from_bits(val as u8)
        }
        #[doc = "Alpha mode. These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless"]
        #[inline(always)]
        pub fn set_am(&mut self, val: super::vals::FgpfccrAm) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Chroma Sub-Sampling. These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless"]
        #[inline(always)]
        pub const fn css(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Chroma Sub-Sampling. These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless"]
        #[inline(always)]
        pub fn set_css(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn ai(&self) -> super::vals::FgpfccrAi {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::FgpfccrAi::from_bits(val as u8)
        }
        #[doc = "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_ai(&mut self, val: super::vals::FgpfccrAi) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn rbs(&self) -> super::vals::FgpfccrRbs {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::FgpfccrRbs::from_bits(val as u8)
        }
        #[doc = "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_rbs(&mut self, val: super::vals::FgpfccrRbs) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Alpha value. These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\\[1:0\\]
bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only."]
        #[inline(always)]
        pub const fn alpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Alpha value. These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\\[1:0\\]
bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only."]
        #[inline(always)]
        pub fn set_alpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fgpfccr {
        #[inline(always)]
        fn default() -> Fgpfccr {
            Fgpfccr(0)
        }
    }
    impl core::fmt::Debug for Fgpfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fgpfccr")
                .field("cm", &self.cm())
                .field("ccm", &self.ccm())
                .field("start", &self.start())
                .field("cs", &self.cs())
                .field("am", &self.am())
                .field("css", &self.css())
                .field("ai", &self.ai())
                .field("rbs", &self.rbs())
                .field("alpha", &self.alpha())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fgpfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fgpfccr {{ cm: {:?}, ccm: {:?}, start: {:?}, cs: {=u8:?}, am: {:?}, css: {=u8:?}, ai: {:?}, rbs: {:?}, alpha: {=u8:?} }}" , self . cm () , self . ccm () , self . start () , self . cs () , self . am () , self . css () , self . ai () , self . rbs () , self . alpha ())
        }
    }
    #[doc = "DMA2D interrupt flag clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifcr(pub u32);
    impl Ifcr {
        #[doc = "Clear transfer error interrupt flag. Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub const fn cteif(&self) -> super::vals::Cteif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Cteif::from_bits(val as u8)
        }
        #[doc = "Clear transfer error interrupt flag. Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub fn set_cteif(&mut self, val: super::vals::Cteif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear transfer complete interrupt flag. Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub const fn ctcif(&self) -> super::vals::Ctcif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Ctcif::from_bits(val as u8)
        }
        #[doc = "Clear transfer complete interrupt flag. Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub fn set_ctcif(&mut self, val: super::vals::Ctcif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear transfer watermark interrupt flag. Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub const fn ctwif(&self) -> super::vals::Ctwif {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ctwif::from_bits(val as u8)
        }
        #[doc = "Clear transfer watermark interrupt flag. Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub fn set_ctwif(&mut self, val: super::vals::Ctwif) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear CLUT access error interrupt flag. Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub const fn caecif(&self) -> super::vals::Caecif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Caecif::from_bits(val as u8)
        }
        #[doc = "Clear CLUT access error interrupt flag. Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub fn set_caecif(&mut self, val: super::vals::Caecif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear CLUT transfer complete interrupt flag. Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub const fn cctcif(&self) -> super::vals::Cctcif {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Cctcif::from_bits(val as u8)
        }
        #[doc = "Clear CLUT transfer complete interrupt flag. Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub fn set_cctcif(&mut self, val: super::vals::Cctcif) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear configuration error interrupt flag. Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub const fn cceif(&self) -> super::vals::Cceif {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Cceif::from_bits(val as u8)
        }
        #[doc = "Clear configuration error interrupt flag. Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
        #[inline(always)]
        pub fn set_cceif(&mut self, val: super::vals::Cceif) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
                .field("cteif", &self.cteif())
                .field("ctcif", &self.ctcif())
                .field("ctwif", &self.ctwif())
                .field("caecif", &self.caecif())
                .field("cctcif", &self.cctcif())
                .field("cceif", &self.cceif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ifcr {{ cteif: {:?}, ctcif: {:?}, ctwif: {:?}, caecif: {:?}, cctcif: {:?}, cceif: {:?} }}",
                self.cteif(),
                self.ctcif(),
                self.ctwif(),
                self.caecif(),
                self.cctcif(),
                self.cceif()
            )
        }
    }
    #[doc = "DMA2D Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Transfer error interrupt flag. This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading)."]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error interrupt flag. This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading)."]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transfer complete interrupt flag. This bit is set when a DMA2D transfer operation is complete (data transfer only)."]
        #[inline(always)]
        pub const fn tcif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt flag. This bit is set when a DMA2D transfer operation is complete (data transfer only)."]
        #[inline(always)]
        pub fn set_tcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer watermark interrupt flag. This bit is set when the last pixel of the watermarked line has been transferred."]
        #[inline(always)]
        pub const fn twif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer watermark interrupt flag. This bit is set when the last pixel of the watermarked line has been transferred."]
        #[inline(always)]
        pub fn set_twif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CLUT access error interrupt flag. This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D."]
        #[inline(always)]
        pub const fn caeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT access error interrupt flag. This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D."]
        #[inline(always)]
        pub fn set_caeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CLUT transfer complete interrupt flag. This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete."]
        #[inline(always)]
        pub const fn ctcif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT transfer complete interrupt flag. This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete."]
        #[inline(always)]
        pub fn set_ctcif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Configuration error interrupt flag. This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed."]
        #[inline(always)]
        pub const fn ceif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration error interrupt flag. This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed."]
        #[inline(always)]
        pub fn set_ceif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("teif", &self.teif())
                .field("tcif", &self.tcif())
                .field("twif", &self.twif())
                .field("caeif", &self.caeif())
                .field("ctcif", &self.ctcif())
                .field("ceif", &self.ceif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ teif: {=bool:?}, tcif: {=bool:?}, twif: {=bool:?}, caeif: {=bool:?}, ctcif: {=bool:?}, ceif: {=bool:?} }}" , self . teif () , self . tcif () , self . twif () , self . caeif () , self . ctcif () , self . ceif ())
        }
    }
    #[doc = "DMA2D line watermark register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lwr(pub u32);
    impl Lwr {
        #[doc = "Line watermark. These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn lw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line watermark. These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_lw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lwr {
        #[inline(always)]
        fn default() -> Lwr {
            Lwr(0)
        }
    }
    impl core::fmt::Debug for Lwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lwr").field("lw", &self.lw()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lwr {{ lw: {=u16:?} }}", self.lw())
        }
    }
    #[doc = "DMA2D number of line register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nlr(pub u32);
    impl Nlr {
        #[doc = "Number of lines. Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn nl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of lines. Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_nl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Pixel per lines. Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
        #[inline(always)]
        pub const fn pl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Pixel per lines. Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
        #[inline(always)]
        pub fn set_pl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Nlr {
        #[inline(always)]
        fn default() -> Nlr {
            Nlr(0)
        }
    }
    impl core::fmt::Debug for Nlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nlr")
                .field("nl", &self.nl())
                .field("pl", &self.pl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nlr {{ nl: {=u16:?}, pl: {=u16:?} }}", self.nl(), self.pl())
        }
    }
    #[doc = "DMA2D output color register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ocolr(pub u32);
    impl Ocolr {
        #[doc = "Color. Color in the format specified by color mode in OPFCCR (16, 24 or 32 bits). These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn color(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Color. Color in the format specified by color mode in OPFCCR (16, 24 or 32 bits). These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_color(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ocolr {
        #[inline(always)]
        fn default() -> Ocolr {
            Ocolr(0)
        }
    }
    impl core::fmt::Debug for Ocolr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ocolr").field("color", &self.color()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ocolr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ocolr {{ color: {=u32:?} }}", self.color())
        }
    }
    #[doc = "DMA2D output memory address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Omar(pub u32);
    impl Omar {
        #[doc = "Memory Address. Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
        #[inline(always)]
        pub const fn ma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory Address. Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
        #[inline(always)]
        pub fn set_ma(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Omar {
        #[inline(always)]
        fn default() -> Omar {
            Omar(0)
        }
    }
    impl core::fmt::Debug for Omar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Omar").field("ma", &self.ma()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Omar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Omar {{ ma: {=u32:?} }}", self.ma())
        }
    }
    #[doc = "DMA2D output offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oor(pub u32);
    impl Oor {
        #[doc = "Line offset. Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line offset. Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
        #[inline(always)]
        pub fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Oor {
        #[inline(always)]
        fn default() -> Oor {
            Oor(0)
        }
    }
    impl core::fmt::Debug for Oor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oor").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oor {{ lo: {=u16:?} }}", self.lo())
        }
    }
    #[doc = "DMA2D output PFC control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opfccr(pub u32);
    impl Opfccr {
        #[doc = "Color mode. These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub const fn cm(&self) -> super::vals::OpfccrCm {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::OpfccrCm::from_bits(val as u8)
        }
        #[doc = "Color mode. These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
        #[inline(always)]
        pub fn set_cm(&mut self, val: super::vals::OpfccrCm) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Swap Bytes"]
        #[inline(always)]
        pub const fn sb(&self) -> super::vals::Sb {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Sb::from_bits(val as u8)
        }
        #[doc = "Swap Bytes"]
        #[inline(always)]
        pub fn set_sb(&mut self, val: super::vals::Sb) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn ai(&self) -> super::vals::OpfccrAi {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OpfccrAi::from_bits(val as u8)
        }
        #[doc = "Alpha inverted. This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_ai(&mut self, val: super::vals::OpfccrAi) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub const fn rbs(&self) -> super::vals::OpfccrRbs {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OpfccrRbs::from_bits(val as u8)
        }
        #[doc = "Red blue swap. This bit allows to swap the R and B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
        #[inline(always)]
        pub fn set_rbs(&mut self, val: super::vals::OpfccrRbs) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Opfccr {
        #[inline(always)]
        fn default() -> Opfccr {
            Opfccr(0)
        }
    }
    impl core::fmt::Debug for Opfccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opfccr")
                .field("cm", &self.cm())
                .field("sb", &self.sb())
                .field("ai", &self.ai())
                .field("rbs", &self.rbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opfccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opfccr {{ cm: {:?}, sb: {:?}, ai: {:?}, rbs: {:?} }}",
                self.cm(),
                self.sb(),
                self.ai(),
                self.rbs()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Abort {
        _RESERVED_0 = 0x0,
        #[doc = "Transfer abort requested"]
        ABORT_REQUEST = 0x01,
    }
    impl Abort {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Abort {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Abort {
        #[inline(always)]
        fn from(val: u8) -> Abort {
            Abort::from_bits(val)
        }
    }
    impl From<Abort> for u8 {
        #[inline(always)]
        fn from(val: Abort) -> u8 {
            Abort::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrAi {
        #[doc = "Regular alpha"]
        REGULAR_ALPHA = 0x0,
        #[doc = "Inverted alpha"]
        INVERTED_ALPHA = 0x01,
    }
    impl BgpfccrAi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrAi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrAi {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrAi {
            BgpfccrAi::from_bits(val)
        }
    }
    impl From<BgpfccrAi> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrAi) -> u8 {
            BgpfccrAi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrAm {
        #[doc = "No modification of alpha channel"]
        NO_MODIFY = 0x0,
        #[doc = "Replace with value in ALPHA\\[7:0\\]"]
        REPLACE = 0x01,
        #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
        MULTIPLY = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl BgpfccrAm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrAm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrAm {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrAm {
            BgpfccrAm::from_bits(val)
        }
    }
    impl From<BgpfccrAm> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrAm) -> u8 {
            BgpfccrAm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrCcm {
        #[doc = "CLUT color format ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "CLUT color format RGB888"]
        RGB888 = 0x01,
    }
    impl BgpfccrCcm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrCcm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrCcm {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrCcm {
            BgpfccrCcm::from_bits(val)
        }
    }
    impl From<BgpfccrCcm> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrCcm) -> u8 {
            BgpfccrCcm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrCm {
        #[doc = "Color mode ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "Color mode RGB888"]
        RGB888 = 0x01,
        #[doc = "Color mode RGB565"]
        RGB565 = 0x02,
        #[doc = "Color mode ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "Color mode ARGB4444"]
        ARGB4444 = 0x04,
        #[doc = "Color mode L8"]
        L8 = 0x05,
        #[doc = "Color mode AL44"]
        AL44 = 0x06,
        #[doc = "Color mode AL88"]
        AL88 = 0x07,
        #[doc = "Color mode L4"]
        L4 = 0x08,
        #[doc = "Color mode A8"]
        A8 = 0x09,
        #[doc = "Color mode A4"]
        A4 = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl BgpfccrCm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrCm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrCm {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrCm {
            BgpfccrCm::from_bits(val)
        }
    }
    impl From<BgpfccrCm> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrCm) -> u8 {
            BgpfccrCm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrRbs {
        #[doc = "No red blue swap (RGB or ARGB)"]
        REGULAR = 0x0,
        #[doc = "Red blue swap (BGR or ABGR)"]
        SWAP = 0x01,
    }
    impl BgpfccrRbs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrRbs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrRbs {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrRbs {
            BgpfccrRbs::from_bits(val)
        }
    }
    impl From<BgpfccrRbs> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrRbs) -> u8 {
            BgpfccrRbs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BgpfccrStart {
        _RESERVED_0 = 0x0,
        #[doc = "Start the automatic loading of the CLUT"]
        START = 0x01,
    }
    impl BgpfccrStart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BgpfccrStart {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BgpfccrStart {
        #[inline(always)]
        fn from(val: u8) -> BgpfccrStart {
            BgpfccrStart::from_bits(val)
        }
    }
    impl From<BgpfccrStart> for u8 {
        #[inline(always)]
        fn from(val: BgpfccrStart) -> u8 {
            BgpfccrStart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Caecif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the CAEIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Caecif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Caecif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Caecif {
        #[inline(always)]
        fn from(val: u8) -> Caecif {
            Caecif::from_bits(val)
        }
    }
    impl From<Caecif> for u8 {
        #[inline(always)]
        fn from(val: Caecif) -> u8 {
            Caecif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cceif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the CEIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cceif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cceif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cceif {
        #[inline(always)]
        fn from(val: u8) -> Cceif {
            Cceif::from_bits(val)
        }
    }
    impl From<Cceif> for u8 {
        #[inline(always)]
        fn from(val: Cceif) -> u8 {
            Cceif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cctcif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the CTCIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cctcif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cctcif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cctcif {
        #[inline(always)]
        fn from(val: u8) -> Cctcif {
            Cctcif::from_bits(val)
        }
    }
    impl From<Cctcif> for u8 {
        #[inline(always)]
        fn from(val: Cctcif) -> u8 {
            Cctcif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CrStart {
        _RESERVED_0 = 0x0,
        #[doc = "Launch the DMA2D"]
        START = 0x01,
    }
    impl CrStart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CrStart {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CrStart {
        #[inline(always)]
        fn from(val: u8) -> CrStart {
            CrStart::from_bits(val)
        }
    }
    impl From<CrStart> for u8 {
        #[inline(always)]
        fn from(val: CrStart) -> u8 {
            CrStart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ctcif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the TCIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Ctcif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ctcif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ctcif {
        #[inline(always)]
        fn from(val: u8) -> Ctcif {
            Ctcif::from_bits(val)
        }
    }
    impl From<Ctcif> for u8 {
        #[inline(always)]
        fn from(val: Ctcif) -> u8 {
            Ctcif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cteif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the TEIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cteif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cteif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cteif {
        #[inline(always)]
        fn from(val: u8) -> Cteif {
            Cteif::from_bits(val)
        }
    }
    impl From<Cteif> for u8 {
        #[inline(always)]
        fn from(val: Cteif) -> u8 {
            Cteif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ctwif {
        _RESERVED_0 = 0x0,
        #[doc = "Clear the TWIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Ctwif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ctwif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ctwif {
        #[inline(always)]
        fn from(val: u8) -> Ctwif {
            Ctwif::from_bits(val)
        }
    }
    impl From<Ctwif> for u8 {
        #[inline(always)]
        fn from(val: Ctwif) -> u8 {
            Ctwif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrAi {
        #[doc = "Regular alpha"]
        REGULAR_ALPHA = 0x0,
        #[doc = "Inverted alpha"]
        INVERTED_ALPHA = 0x01,
    }
    impl FgpfccrAi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrAi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrAi {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrAi {
            FgpfccrAi::from_bits(val)
        }
    }
    impl From<FgpfccrAi> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrAi) -> u8 {
            FgpfccrAi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrAm {
        #[doc = "No modification of alpha channel"]
        NO_MODIFY = 0x0,
        #[doc = "Replace with value in ALPHA\\[7:0\\]"]
        REPLACE = 0x01,
        #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
        MULTIPLY = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl FgpfccrAm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrAm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrAm {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrAm {
            FgpfccrAm::from_bits(val)
        }
    }
    impl From<FgpfccrAm> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrAm) -> u8 {
            FgpfccrAm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrCcm {
        #[doc = "CLUT color format ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "CLUT color format RGB888"]
        RGB888 = 0x01,
    }
    impl FgpfccrCcm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrCcm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrCcm {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrCcm {
            FgpfccrCcm::from_bits(val)
        }
    }
    impl From<FgpfccrCcm> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrCcm) -> u8 {
            FgpfccrCcm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrCm {
        #[doc = "Color mode ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "Color mode RGB888"]
        RGB888 = 0x01,
        #[doc = "Color mode RGB565"]
        RGB565 = 0x02,
        #[doc = "Color mode ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "Color mode ARGB4444"]
        ARGB4444 = 0x04,
        #[doc = "Color mode L8"]
        L8 = 0x05,
        #[doc = "Color mode AL44"]
        AL44 = 0x06,
        #[doc = "Color mode AL88"]
        AL88 = 0x07,
        #[doc = "Color mode L4"]
        L4 = 0x08,
        #[doc = "Color mode A8"]
        A8 = 0x09,
        #[doc = "Color mode A4"]
        A4 = 0x0a,
        #[doc = "Color mode YCbCr"]
        YCB_CR = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl FgpfccrCm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrCm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrCm {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrCm {
            FgpfccrCm::from_bits(val)
        }
    }
    impl From<FgpfccrCm> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrCm) -> u8 {
            FgpfccrCm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrRbs {
        #[doc = "No red blue swap (RGB or ARGB)"]
        REGULAR = 0x0,
        #[doc = "red blue swap (BGR or ABGR)"]
        SWAP = 0x01,
    }
    impl FgpfccrRbs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrRbs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrRbs {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrRbs {
            FgpfccrRbs::from_bits(val)
        }
    }
    impl From<FgpfccrRbs> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrRbs) -> u8 {
            FgpfccrRbs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FgpfccrStart {
        _RESERVED_0 = 0x0,
        #[doc = "Start the automatic loading of the CLUT"]
        START = 0x01,
    }
    impl FgpfccrStart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FgpfccrStart {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FgpfccrStart {
        #[inline(always)]
        fn from(val: u8) -> FgpfccrStart {
            FgpfccrStart::from_bits(val)
        }
    }
    impl From<FgpfccrStart> for u8 {
        #[inline(always)]
        fn from(val: FgpfccrStart) -> u8 {
            FgpfccrStart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lom {
        #[doc = "Line offsets expressed in pixels"]
        PIXELS = 0x0,
        #[doc = "Line offsets expressed in bytes"]
        BYTES = 0x01,
    }
    impl Lom {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lom {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lom {
        #[inline(always)]
        fn from(val: u8) -> Lom {
            Lom::from_bits(val)
        }
    }
    impl From<Lom> for u8 {
        #[inline(always)]
        fn from(val: Lom) -> u8 {
            Lom::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Memory-to-memory (FG fetch only)"]
        MEMORY_TO_MEMORY = 0x0,
        #[doc = "Memory-to-memory with PFC (pixel format converter) (FG fetch only with FG PFC active)"]
        MEMORY_TO_MEMORY_PFC = 0x01,
        #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
        MEMORY_TO_MEMORY_PFCBLENDING = 0x02,
        #[doc = "Register-to-memory (no FG nor BG, only output stage active)"]
        REGISTER_TO_MEMORY = 0x03,
        #[doc = "Memory-to-memory with blending and fixed color FG (BG fetch only with FG and BG PFC active)"]
        MEMORY_TO_MEMORY_PFCBLENDING_FIXED_COLOR_FG = 0x04,
        #[doc = "Memory-to-memory with blending and fixed color BG (FG fetch only with FG and BG PFC active)"]
        MEMORY_TO_MEMORY_PFCBLENDING_FIXED_COLOR_BG = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OpfccrAi {
        #[doc = "Regular alpha"]
        REGULAR_ALPHA = 0x0,
        #[doc = "Inverted alpha"]
        INVERTED_ALPHA = 0x01,
    }
    impl OpfccrAi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpfccrAi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpfccrAi {
        #[inline(always)]
        fn from(val: u8) -> OpfccrAi {
            OpfccrAi::from_bits(val)
        }
    }
    impl From<OpfccrAi> for u8 {
        #[inline(always)]
        fn from(val: OpfccrAi) -> u8 {
            OpfccrAi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OpfccrCm {
        #[doc = "ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "RGB888"]
        RGB888 = 0x01,
        #[doc = "RGB565"]
        RGB565 = 0x02,
        #[doc = "ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "ARGB4444"]
        ARGB4444 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl OpfccrCm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpfccrCm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpfccrCm {
        #[inline(always)]
        fn from(val: u8) -> OpfccrCm {
            OpfccrCm::from_bits(val)
        }
    }
    impl From<OpfccrCm> for u8 {
        #[inline(always)]
        fn from(val: OpfccrCm) -> u8 {
            OpfccrCm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OpfccrRbs {
        #[doc = "No red blue swap (RGB or ARGB)"]
        REGULAR = 0x0,
        #[doc = "Red blue swap (BGR or ABGR)"]
        SWAP = 0x01,
    }
    impl OpfccrRbs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpfccrRbs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpfccrRbs {
        #[inline(always)]
        fn from(val: u8) -> OpfccrRbs {
            OpfccrRbs::from_bits(val)
        }
    }
    impl From<OpfccrRbs> for u8 {
        #[inline(always)]
        fn from(val: OpfccrRbs) -> u8 {
            OpfccrRbs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sb {
        #[doc = "Regular byte order"]
        REGULAR = 0x0,
        #[doc = "Bytes are swapped two by two"]
        SWAP_BYTES = 0x01,
    }
    impl Sb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sb {
        #[inline(always)]
        fn from(val: u8) -> Sb {
            Sb::from_bits(val)
        }
    }
    impl From<Sb> for u8 {
        #[inline(always)]
        fn from(val: Sb) -> u8 {
            Sb::to_bits(val)
        }
    }
}
