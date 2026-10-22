#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "VERSION_ID register."]
    #[inline(always)]
    pub const fn version_id(self) -> crate::common::Reg<regs::VersionId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CONF register."]
    #[inline(always)]
    pub const fn conf(self) -> crate::common::Reg<regs::Conf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "CTRL register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Occasionnal mode control register."]
    #[inline(always)]
    pub const fn ocm_ctrl(self) -> crate::common::Reg<regs::OcmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PGA configuration register."]
    #[inline(always)]
    pub const fn pga_conf(self) -> crate::common::Reg<regs::PgaConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SWITCH register."]
    #[inline(always)]
    pub const fn switch(self) -> crate::common::Reg<regs::Switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Decimation filter configuration register."]
    #[inline(always)]
    pub const fn df_conf(self) -> crate::common::Reg<regs::DfConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DS_CONF register."]
    #[inline(always)]
    pub const fn ds_conf(self) -> crate::common::Reg<regs::DsConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SEQ_1 register."]
    #[inline(always)]
    pub const fn seq_1(self) -> crate::common::Reg<regs::Seq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SEQ_2 register."]
    #[inline(always)]
    pub const fn seq_2(self) -> crate::common::Reg<regs::Seq2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "COMP_1 register."]
    #[inline(always)]
    pub const fn comp_1(self) -> crate::common::Reg<regs::Comp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "COMP_2 register."]
    #[inline(always)]
    pub const fn comp_2(self) -> crate::common::Reg<regs::Comp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "COMP_3 register."]
    #[inline(always)]
    pub const fn comp_3(self) -> crate::common::Reg<regs::Comp3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "COMP_4 register."]
    #[inline(always)]
    pub const fn comp_4(self) -> crate::common::Reg<regs::Comp4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "COMP_SEL register."]
    #[inline(always)]
    pub const fn comp_sel(self) -> crate::common::Reg<regs::CompSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "WD_TH register."]
    #[inline(always)]
    pub const fn wd_th(self) -> crate::common::Reg<regs::WdTh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "WD_CONF register."]
    #[inline(always)]
    pub const fn wd_conf(self) -> crate::common::Reg<regs::WdConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DS_DATAOUT register."]
    #[inline(always)]
    pub const fn ds_dataout(self) -> crate::common::Reg<regs::DsDataout, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Decimation filter Data output register."]
    #[inline(always)]
    pub const fn df_dataout(self) -> crate::common::Reg<regs::DfDataout, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "IRQ_STATUS register."]
    #[inline(always)]
    pub const fn irq_status(self) -> crate::common::Reg<regs::IrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "IRQ_ENABLE register."]
    #[inline(always)]
    pub const fn irq_enable(self) -> crate::common::Reg<regs::IrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it."]
    #[inline(always)]
    pub const fn timer_conf(self) -> crate::common::Reg<regs::TimerConf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
}
pub mod regs {
    #[doc = "COMP_1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comp1(pub u32);
    impl Comp1 {
        #[doc = "GAIN1\\[11:0\\]: first calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub const fn gain1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "GAIN1\\[11:0\\]: first calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub fn set_gain1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "OFFSET1\\[7:0\\]: first calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub const fn offset1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "OFFSET1\\[7:0\\]: first calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub fn set_offset1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for Comp1 {
        #[inline(always)]
        fn default() -> Comp1 {
            Comp1(0)
        }
    }
    impl core::fmt::Debug for Comp1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Comp1")
                .field("gain1", &self.gain1())
                .field("offset1", &self.offset1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Comp1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Comp1 {{ gain1: {=u16:?}, offset1: {=u8:?} }}",
                self.gain1(),
                self.offset1()
            )
        }
    }
    #[doc = "COMP_2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comp2(pub u32);
    impl Comp2 {
        #[doc = "GAIN2\\[11:0\\]: second calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub const fn gain2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "GAIN2\\[11:0\\]: second calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub fn set_gain2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "OFFSET2\\[7:0\\]: second calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub const fn offset2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "OFFSET2\\[7:0\\]: second calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub fn set_offset2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for Comp2 {
        #[inline(always)]
        fn default() -> Comp2 {
            Comp2(0)
        }
    }
    impl core::fmt::Debug for Comp2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Comp2")
                .field("gain2", &self.gain2())
                .field("offset2", &self.offset2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Comp2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Comp2 {{ gain2: {=u16:?}, offset2: {=u8:?} }}",
                self.gain2(),
                self.offset2()
            )
        }
    }
    #[doc = "COMP_3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comp3(pub u32);
    impl Comp3 {
        #[doc = "GAIN3\\[11:0\\]: third calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub const fn gain3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "GAIN3\\[11:0\\]: third calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub fn set_gain3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "OFFSET3\\[7:0\\]: third calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub const fn offset3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "OFFSET3\\[7:0\\]: third calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub fn set_offset3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for Comp3 {
        #[inline(always)]
        fn default() -> Comp3 {
            Comp3(0)
        }
    }
    impl core::fmt::Debug for Comp3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Comp3")
                .field("gain3", &self.gain3())
                .field("offset3", &self.offset3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Comp3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Comp3 {{ gain3: {=u16:?}, offset3: {=u8:?} }}",
                self.gain3(),
                self.offset3()
            )
        }
    }
    #[doc = "COMP_4 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Comp4(pub u32);
    impl Comp4 {
        #[doc = "GAIN4\\[11:0\\]: fourth calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub const fn gain4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "GAIN4\\[11:0\\]: fourth calibration point: gain AUXADC_GAIN_1V2\\[11:0\\]."]
        #[inline(always)]
        pub fn set_gain4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "OFFSET4\\[7:0\\]: fourth calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub const fn offset4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "OFFSET4\\[7:0\\]: fourth calibration point: offset compensation\\[7:0\\]
with sign."]
        #[inline(always)]
        pub fn set_offset4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for Comp4 {
        #[inline(always)]
        fn default() -> Comp4 {
            Comp4(0)
        }
    }
    impl core::fmt::Debug for Comp4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Comp4")
                .field("gain4", &self.gain4())
                .field("offset4", &self.offset4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Comp4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Comp4 {{ gain4: {=u16:?}, offset4: {=u8:?} }}",
                self.gain4(),
                self.offset4()
            )
        }
    }
    #[doc = "COMP_SEL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CompSel(pub u32);
    impl CompSel {
        #[doc = "gain / offset used in ADC single negative mode with Vinput range = 1.2V."]
        #[inline(always)]
        pub const fn gain_offset0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC single negative mode with Vinput range = 1.2V."]
        #[inline(always)]
        pub fn set_gain_offset0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "OFFSET_GAIN0\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN0\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "gain / offset used in ADC single positive mode with Vinput range = 1.2V."]
        #[inline(always)]
        pub const fn gain_offset1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC single positive mode with Vinput range = 1.2V."]
        #[inline(always)]
        pub fn set_gain_offset1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "OFFSET_GAIN1\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN1\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "gain / offset used in ADC differential mode with Vinput range = 1.2V."]
        #[inline(always)]
        pub const fn gain_offset2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC differential mode with Vinput range = 1.2V."]
        #[inline(always)]
        pub fn set_gain_offset2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "OFFSET_GAIN2\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN2\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "gain / offset used in ADC single negative mode with Vinput range = 2.4V."]
        #[inline(always)]
        pub const fn gain_offset3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC single negative mode with Vinput range = 2.4V."]
        #[inline(always)]
        pub fn set_gain_offset3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "OFFSET_GAIN3\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN3\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "gain / offset used in ADC single positive mode with Vinput range = 2.4V."]
        #[inline(always)]
        pub const fn gain_offset4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC single positive mode with Vinput range = 2.4V."]
        #[inline(always)]
        pub fn set_gain_offset4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "OFFSET_GAIN4\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN4\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "gain / offset used in ADC differential mode with Vinput range = 2.4V."]
        #[inline(always)]
        pub const fn gain_offset5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC differential mode with Vinput range = 2.4V."]
        #[inline(always)]
        pub fn set_gain_offset5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "OFFSET_GAIN5\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN5\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "gain / offset used in ADC single negative mode with Vinput range = 3.6V."]
        #[inline(always)]
        pub const fn gain_offset6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC single negative mode with Vinput range = 3.6V."]
        #[inline(always)]
        pub fn set_gain_offset6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "OFFSET_GAIN6\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN6\\[1:0\\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "gain / offset used in ADC single positive mode with Vinput range = 3.6V."]
        #[inline(always)]
        pub const fn gain_offset7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC single positive mode with Vinput range = 3.6V."]
        #[inline(always)]
        pub fn set_gain_offset7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "OFFSET_GAIN7\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN7\\[1:0\\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "gain / offset used in ADC differential mode with Vinput range = 3.6V."]
        #[inline(always)]
        pub const fn gain_offset8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "gain / offset used in ADC differential mode with Vinput range = 3.6V."]
        #[inline(always)]
        pub fn set_gain_offset8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "OFFSET_GAIN8\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub const fn offset_gain8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "OFFSET_GAIN8\\[1:0\\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4."]
        #[inline(always)]
        pub fn set_offset_gain8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for CompSel {
        #[inline(always)]
        fn default() -> CompSel {
            CompSel(0)
        }
    }
    impl core::fmt::Debug for CompSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CompSel")
                .field("gain_offset0", &self.gain_offset0())
                .field("offset_gain0", &self.offset_gain0())
                .field("gain_offset1", &self.gain_offset1())
                .field("offset_gain1", &self.offset_gain1())
                .field("gain_offset2", &self.gain_offset2())
                .field("offset_gain2", &self.offset_gain2())
                .field("gain_offset3", &self.gain_offset3())
                .field("offset_gain3", &self.offset_gain3())
                .field("gain_offset4", &self.gain_offset4())
                .field("offset_gain4", &self.offset_gain4())
                .field("gain_offset5", &self.gain_offset5())
                .field("offset_gain5", &self.offset_gain5())
                .field("gain_offset6", &self.gain_offset6())
                .field("offset_gain6", &self.offset_gain6())
                .field("gain_offset7", &self.gain_offset7())
                .field("offset_gain7", &self.offset_gain7())
                .field("gain_offset8", &self.gain_offset8())
                .field("offset_gain8", &self.offset_gain8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CompSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CompSel {{ gain_offset0: {=u8:?}, offset_gain0: {=u8:?}, gain_offset1: {=u8:?}, offset_gain1: {=u8:?}, gain_offset2: {=u8:?}, offset_gain2: {=u8:?}, gain_offset3: {=u8:?}, offset_gain3: {=u8:?}, gain_offset4: {=u8:?}, offset_gain4: {=u8:?}, gain_offset5: {=u8:?}, offset_gain5: {=u8:?}, gain_offset6: {=u8:?}, offset_gain6: {=u8:?}, gain_offset7: {=u8:?}, offset_gain7: {=u8:?}, gain_offset8: {=u8:?}, offset_gain8: {=u8:?} }}" , self . gain_offset0 () , self . offset_gain0 () , self . gain_offset1 () , self . offset_gain1 () , self . gain_offset2 () , self . offset_gain2 () , self . gain_offset3 () , self . offset_gain3 () , self . gain_offset4 () , self . offset_gain4 () , self . gain_offset5 () , self . offset_gain5 () , self . gain_offset6 () , self . offset_gain6 () , self . gain_offset7 () , self . offset_gain7 () , self . gain_offset8 () , self . offset_gain8 ())
        }
    }
    #[doc = "CONF register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Conf(pub u32);
    impl Conf {
        #[doc = "CONT: regular sequence runs continuously when ADC mode is enabled: 0: enable the single conversion: when the sequence is over, the conversion stops 1: enable the continuous conversion: when the sequence is over, the sequence starts again until the software sets the CTRL.STOP_OP_MODE bit."]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CONT: regular sequence runs continuously when ADC mode is enabled: 0: enable the single conversion: when the sequence is over, the conversion stops 1: enable the continuous conversion: when the sequence is over, the sequence starts again until the software sets the CTRL.STOP_OP_MODE bit."]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SEQUENCE: enable the sequence mode (active by default): 0: sequence mode is disabled, only SEQ0 is selected 1: sequence mode is enabled, conversions from SEQ0 to SEQx with x=SEQ_LEN Note: clearing this bit is equivalent to SEQUENCE=1 and SEQ_LEN=0000. Ideally, this bit can be kept high as redundant with keeping high and setting SEQ_LEN=0000."]
        #[inline(always)]
        pub const fn sequence(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SEQUENCE: enable the sequence mode (active by default): 0: sequence mode is disabled, only SEQ0 is selected 1: sequence mode is enabled, conversions from SEQ0 to SEQx with x=SEQ_LEN Note: clearing this bit is equivalent to SEQUENCE=1 and SEQ_LEN=0000. Ideally, this bit can be kept high as redundant with keeping high and setting SEQ_LEN=0000."]
        #[inline(always)]
        pub fn set_sequence(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SEQ_LEN\\[3:0\\]: number of conversions in a regular sequence: 0000: 1 conversion, starting from SEQ0 0001: 2 conversions, starting from SEQ0 ... 1111: 16 conversions, starting from SEQ0."]
        #[inline(always)]
        pub const fn seq_len(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ_LEN\\[3:0\\]: number of conversions in a regular sequence: 0000: 1 conversion, starting from SEQ0 0001: 2 conversions, starting from SEQ0 ... 1111: 16 conversions, starting from SEQ0."]
        #[inline(always)]
        pub fn set_seq_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "SMPS_SYNCHRO_ENA: synchronize the ADC start conversion with a pulse generated by the SMPS: 0: SMPS synchronization is disabled for all ADC clock frequencies 1: SMPS synchronization is enabled (only when ADC clock is 8 MHz or 16 MHz) Note: SMPS_SYNCHRO_ENA must be 0 when the ADC analog clock is 32 MHz or when PWRC_CR5.NOSMPS = 1."]
        #[inline(always)]
        pub const fn smps_synchro_ena(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS_SYNCHRO_ENA: synchronize the ADC start conversion with a pulse generated by the SMPS: 0: SMPS synchronization is disabled for all ADC clock frequencies 1: SMPS synchronization is enabled (only when ADC clock is 8 MHz or 16 MHz) Note: SMPS_SYNCHRO_ENA must be 0 when the ADC analog clock is 32 MHz or when PWRC_CR5.NOSMPS = 1."]
        #[inline(always)]
        pub fn set_smps_synchro_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ADC mode selection (= data path selection)."]
        #[inline(always)]
        pub const fn op_mode(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "ADC mode selection (= data path selection)."]
        #[inline(always)]
        pub fn set_op_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "SAMPLE_RATE_LSB: Sample Rate LSB This field is an extension of SAMPLE_RATE definition in bits 12,11 of CONF register. It impacts the conversion rate of ADC (F_ADC). See SAMPLE_RATE bits for the full description. When this field is set to a value different than 0, SMPS synchronization is not feasible. This value is hidden to the user."]
        #[inline(always)]
        pub const fn sample_rate_lsb(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "SAMPLE_RATE_LSB: Sample Rate LSB This field is an extension of SAMPLE_RATE definition in bits 12,11 of CONF register. It impacts the conversion rate of ADC (F_ADC). See SAMPLE_RATE bits for the full description. When this field is set to a value different than 0, SMPS synchronization is not feasible. This value is hidden to the user."]
        #[inline(always)]
        pub fn set_sample_rate_lsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "SAMPLE_RATE\\[1:0\\]: conversion rate of ADC (F_ADC): F_ADC = F_ADC_CLK/(16 + 16*SAMPLE_RATE_MSB + 4*SAMPLE_RATE + SAMPLE_RATE_LSB),where F_ADC_CLK is the analog ADC clock frequency. By default F_ADC_CLK is 16MHz frequency."]
        #[inline(always)]
        pub const fn sample_rate(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "SAMPLE_RATE\\[1:0\\]: conversion rate of ADC (F_ADC): F_ADC = F_ADC_CLK/(16 + 16*SAMPLE_RATE_MSB + 4*SAMPLE_RATE + SAMPLE_RATE_LSB),where F_ADC_CLK is the analog ADC clock frequency. By default F_ADC_CLK is 16MHz frequency."]
        #[inline(always)]
        pub fn set_sample_rate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "DMA_DS_EN: enable the DMA mode for the Down Sampler data path: 0: DMA mode is disabled 1: DMA mode is enabled."]
        #[inline(always)]
        pub const fn dma_ds_ena(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DMA_DS_EN: enable the DMA mode for the Down Sampler data path: 0: DMA mode is disabled 1: DMA mode is enabled."]
        #[inline(always)]
        pub fn set_dma_ds_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "enable DMA mode for Decimation Filter data path."]
        #[inline(always)]
        pub const fn dma_df_ena(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "enable DMA mode for Decimation Filter data path."]
        #[inline(always)]
        pub fn set_dma_df_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OVR_DS_CFG: Down Sampler overrun configuration: 0: the previous data is kept, the new one is lost 1: the previous data is lost, the new one is kept."]
        #[inline(always)]
        pub const fn ovr_ds_cfg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OVR_DS_CFG: Down Sampler overrun configuration: 0: the previous data is kept, the new one is lost 1: the previous data is lost, the new one is kept."]
        #[inline(always)]
        pub fn set_ovr_ds_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "decimation overrun configuration."]
        #[inline(always)]
        pub const fn ovr_df_cfg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "decimation overrun configuration."]
        #[inline(always)]
        pub fn set_ovr_df_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BIT_INVERT_SN: invert bit to bit the ADC data output (1's complement) when a single negative input is connected to the ADC: 0: no inversion (default) 1: enable the inversion."]
        #[inline(always)]
        pub const fn bit_invert_sn(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BIT_INVERT_SN: invert bit to bit the ADC data output (1's complement) when a single negative input is connected to the ADC: 0: no inversion (default) 1: enable the inversion."]
        #[inline(always)]
        pub fn set_bit_invert_sn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BIT_INVERT_DIFF: invert bit to bit the ADC data output (1's complement) when a differential input is connected to the ADC: 0: no inversion (default) 1: enable the inversion."]
        #[inline(always)]
        pub const fn bit_invert_diff(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "BIT_INVERT_DIFF: invert bit to bit the ADC data output (1's complement) when a differential input is connected to the ADC: 0: no inversion (default) 1: enable the inversion."]
        #[inline(always)]
        pub fn set_bit_invert_diff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC_CONT_1V2: select the input sampling method: 0: sampling only at conversion start (default) 1: sampling starts at the end of conversion."]
        #[inline(always)]
        pub const fn adc_cont_1v2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC_CONT_1V2: select the input sampling method: 0: sampling only at conversion start (default) 1: sampling starts at the end of conversion."]
        #[inline(always)]
        pub fn set_adc_cont_1v2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "possibility to keep the VBIAS_PRECH enabled to deactivate the filter."]
        #[inline(always)]
        pub const fn vbias_prech_force(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "possibility to keep the VBIAS_PRECH enabled to deactivate the filter."]
        #[inline(always)]
        pub fn set_vbias_prech_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAMPLE_RATE_MSB: Sample Rate MSB This field is an extension of SAMPLE_RATE definition in bits 12,11 of CONF register. It impacts the conversion rate of ADC (F_ADC). See SAMPLE_RATE bits for the full description."]
        #[inline(always)]
        pub const fn sample_rate_msb(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "SAMPLE_RATE_MSB: Sample Rate MSB This field is an extension of SAMPLE_RATE definition in bits 12,11 of CONF register. It impacts the conversion rate of ADC (F_ADC). See SAMPLE_RATE bits for the full description."]
        #[inline(always)]
        pub fn set_sample_rate_msb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
    }
    impl Default for Conf {
        #[inline(always)]
        fn default() -> Conf {
            Conf(0)
        }
    }
    impl core::fmt::Debug for Conf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Conf")
                .field("cont", &self.cont())
                .field("sequence", &self.sequence())
                .field("seq_len", &self.seq_len())
                .field("smps_synchro_ena", &self.smps_synchro_ena())
                .field("op_mode", &self.op_mode())
                .field("sample_rate_lsb", &self.sample_rate_lsb())
                .field("sample_rate", &self.sample_rate())
                .field("dma_ds_ena", &self.dma_ds_ena())
                .field("dma_df_ena", &self.dma_df_ena())
                .field("ovr_ds_cfg", &self.ovr_ds_cfg())
                .field("ovr_df_cfg", &self.ovr_df_cfg())
                .field("bit_invert_sn", &self.bit_invert_sn())
                .field("bit_invert_diff", &self.bit_invert_diff())
                .field("adc_cont_1v2", &self.adc_cont_1v2())
                .field("vbias_prech_force", &self.vbias_prech_force())
                .field("sample_rate_msb", &self.sample_rate_msb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Conf {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Conf {{ cont: {=bool:?}, sequence: {=bool:?}, seq_len: {=u8:?}, smps_synchro_ena: {=bool:?}, op_mode: {=u8:?}, sample_rate_lsb: {=u8:?}, sample_rate: {=u8:?}, dma_ds_ena: {=bool:?}, dma_df_ena: {=bool:?}, ovr_ds_cfg: {=bool:?}, ovr_df_cfg: {=bool:?}, bit_invert_sn: {=bool:?}, bit_invert_diff: {=bool:?}, adc_cont_1v2: {=bool:?}, vbias_prech_force: {=bool:?}, sample_rate_msb: {=u8:?} }}" , self . cont () , self . sequence () , self . seq_len () , self . smps_synchro_ena () , self . op_mode () , self . sample_rate_lsb () , self . sample_rate () , self . dma_ds_ena () , self . dma_df_ena () , self . ovr_ds_cfg () , self . ovr_df_cfg () , self . bit_invert_sn () , self . bit_invert_diff () , self . adc_cont_1v2 () , self . vbias_prech_force () , self . sample_rate_msb ())
        }
    }
    #[doc = "CTRL register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "ADC_ON_OFF: 0: power off the ADC 1: power on the ADC."]
        #[inline(always)]
        pub const fn adc_on_off(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC_ON_OFF: 0: power off the ADC 1: power on the ADC."]
        #[inline(always)]
        pub fn set_adc_on_off(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "generate a start pulse to initiate an ADC conversion."]
        #[inline(always)]
        pub const fn start_con(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "generate a start pulse to initiate an ADC conversion."]
        #[inline(always)]
        pub fn set_start_con(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware."]
        #[inline(always)]
        pub const fn start_conv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware."]
        #[inline(always)]
        pub fn set_start_conv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "stop the on-going OP_MODE (ADC mode, Analog audio mode, Full."]
        #[inline(always)]
        pub const fn stop_op_mod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "stop the on-going OP_MODE (ADC mode, Analog audio mode, Full."]
        #[inline(always)]
        pub fn set_stop_op_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit."]
        #[inline(always)]
        pub const fn stop_op_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit."]
        #[inline(always)]
        pub fn set_stop_op_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable the digital audio mode (the data path uses. the decimation filter)."]
        #[inline(always)]
        pub const fn dig_aud_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable the digital audio mode (the data path uses. the decimation filter)."]
        #[inline(always)]
        pub fn set_dig_aud_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)."]
        #[inline(always)]
        pub const fn test_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)."]
        #[inline(always)]
        pub fn set_test_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC_LDO_ENA: enable the LDO associated to the ADC block: 0: disable the ADC LDO 1: enable the ADC LDO."]
        #[inline(always)]
        pub const fn adc_ldo_ena(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC_LDO_ENA: enable the LDO associated to the ADC block: 0: disable the ADC LDO 1: enable the ADC LDO."]
        #[inline(always)]
        pub fn set_adc_ldo_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("adc_on_off", &self.adc_on_off())
                .field("start_con", &self.start_con())
                .field("start_conv", &self.start_conv())
                .field("stop_op_mod", &self.stop_op_mod())
                .field("stop_op_mode", &self.stop_op_mode())
                .field("dig_aud_mode", &self.dig_aud_mode())
                .field("test_mode", &self.test_mode())
                .field("adc_ldo_ena", &self.adc_ldo_ena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ adc_on_off: {=bool:?}, start_con: {=bool:?}, start_conv: {=bool:?}, stop_op_mod: {=bool:?}, stop_op_mode: {=bool:?}, dig_aud_mode: {=bool:?}, test_mode: {=bool:?}, adc_ldo_ena: {=bool:?} }}" , self . adc_on_off () , self . start_con () , self . start_conv () , self . stop_op_mod () , self . stop_op_mode () , self . dig_aud_mode () , self . test_mode () , self . adc_ldo_ena ())
        }
    }
    #[doc = "Decimation filter configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfConf(pub u32);
    impl DfConf {
        #[inline(always)]
        pub const fn df_cic_dec_factor(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_df_cic_dec_factor(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "CIC filter decimator half factor."]
        #[inline(always)]
        pub const fn df_cic_dhf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CIC filter decimator half factor."]
        #[inline(always)]
        pub fn set_df_cic_dhf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1.2 fractional interpolator enable."]
        #[inline(always)]
        pub const fn df_itp1p2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1.2 fractional interpolator enable."]
        #[inline(always)]
        pub fn set_df_itp1p2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "select signed/unsigned format for input."]
        #[inline(always)]
        pub const fn df_i_u2s(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "select signed/unsigned format for input."]
        #[inline(always)]
        pub fn set_df_i_u2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "select signed/unsigned format for data output."]
        #[inline(always)]
        pub const fn df_o_s2u(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "select signed/unsigned format for data output."]
        #[inline(always)]
        pub fn set_df_o_s2u(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "select the PDM clock rate."]
        #[inline(always)]
        pub const fn pdm_rate(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "select the PDM clock rate."]
        #[inline(always)]
        pub fn set_pdm_rate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "left/right channel selection on digital microphone."]
        #[inline(always)]
        pub const fn df_microl_rn(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "left/right channel selection on digital microphone."]
        #[inline(always)]
        pub fn set_df_microl_rn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "high pass filter enable."]
        #[inline(always)]
        pub const fn df_hpf_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "high pass filter enable."]
        #[inline(always)]
        pub fn set_df_hpf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "half dynamic enable."]
        #[inline(always)]
        pub const fn df_half_d_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "half dynamic enable."]
        #[inline(always)]
        pub fn set_df_half_d_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for DfConf {
        #[inline(always)]
        fn default() -> DfConf {
            DfConf(0)
        }
    }
    impl core::fmt::Debug for DfConf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DfConf")
                .field("df_cic_dec_factor", &self.df_cic_dec_factor())
                .field("df_cic_dhf", &self.df_cic_dhf())
                .field("df_itp1p2", &self.df_itp1p2())
                .field("df_i_u2s", &self.df_i_u2s())
                .field("df_o_s2u", &self.df_o_s2u())
                .field("pdm_rate", &self.pdm_rate())
                .field("df_microl_rn", &self.df_microl_rn())
                .field("df_hpf_en", &self.df_hpf_en())
                .field("df_half_d_en", &self.df_half_d_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DfConf {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DfConf {{ df_cic_dec_factor: {=u8:?}, df_cic_dhf: {=bool:?}, df_itp1p2: {=bool:?}, df_i_u2s: {=bool:?}, df_o_s2u: {=bool:?}, pdm_rate: {=u8:?}, df_microl_rn: {=bool:?}, df_hpf_en: {=bool:?}, df_half_d_en: {=bool:?} }}" , self . df_cic_dec_factor () , self . df_cic_dhf () , self . df_itp1p2 () , self . df_i_u2s () , self . df_o_s2u () , self . pdm_rate () , self . df_microl_rn () , self . df_hpf_en () , self . df_half_d_en ())
        }
    }
    #[doc = "Decimation filter Data output register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DfDataout(pub u32);
    impl DfDataout {
        #[doc = "contain the converted data at the output of the. decimation filter."]
        #[inline(always)]
        pub const fn df_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "contain the converted data at the output of the. decimation filter."]
        #[inline(always)]
        pub fn set_df_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DfDataout {
        #[inline(always)]
        fn default() -> DfDataout {
            DfDataout(0)
        }
    }
    impl core::fmt::Debug for DfDataout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DfDataout").field("df_data", &self.df_data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DfDataout {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DfDataout {{ df_data: {=u16:?} }}", self.df_data())
        }
    }
    #[doc = "DS_CONF register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DsConf(pub u32);
    impl DsConf {
        #[doc = "DS_RATIO\\[2:0\\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128."]
        #[inline(always)]
        pub const fn ds_ratio(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "DS_RATIO\\[2:0\\]: program the Down Sampler ratio (N factor) 000: ratio = 1, no down sampling (default) 001: ratio = 2 010: ratio = 4 011: ratio = 8 100: ratio = 16 101: ratio = 32 110: ratio = 64 111: ratio = 128."]
        #[inline(always)]
        pub fn set_ds_ratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "DS_WIDTH\\[2:0\\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved."]
        #[inline(always)]
        pub const fn ds_width(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "DS_WIDTH\\[2:0\\]: program the Down Sampler width of data output (DSDTATA) 000: DS_DATA output on 12-bit (default) 001: DS_DATA output on 13-bit 010: DS_DATA output on 14-bit 011: DS_DATA output on 15-bit 100: DS_DATA output on 16-bit 1xx: reserved."]
        #[inline(always)]
        pub fn set_ds_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
    }
    impl Default for DsConf {
        #[inline(always)]
        fn default() -> DsConf {
            DsConf(0)
        }
    }
    impl core::fmt::Debug for DsConf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DsConf")
                .field("ds_ratio", &self.ds_ratio())
                .field("ds_width", &self.ds_width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DsConf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DsConf {{ ds_ratio: {=u8:?}, ds_width: {=u8:?} }}",
                self.ds_ratio(),
                self.ds_width()
            )
        }
    }
    #[doc = "DS_DATAOUT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DsDataout(pub u32);
    impl DsDataout {
        #[doc = "DS_DATA\\[15:0\\]: contain the converted data at the output of the Down Sampler."]
        #[inline(always)]
        pub const fn ds_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DS_DATA\\[15:0\\]: contain the converted data at the output of the Down Sampler."]
        #[inline(always)]
        pub fn set_ds_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DsDataout {
        #[inline(always)]
        fn default() -> DsDataout {
            DsDataout(0)
        }
    }
    impl core::fmt::Debug for DsDataout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DsDataout").field("ds_data", &self.ds_data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DsDataout {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DsDataout {{ ds_data: {=u16:?} }}", self.ds_data())
        }
    }
    #[doc = "IRQ_ENABLE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnable(pub u32);
    impl IrqEnable {
        #[doc = "EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn eoc_irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_eoc_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "(Used in test mode only): End of ADC conversion interrupt enable."]
        #[inline(always)]
        pub const fn eoc_irq_ena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "(Used in test mode only): End of ADC conversion interrupt enable."]
        #[inline(always)]
        pub fn set_eoc_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn eods_irq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_eods_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of conversion interrupt enable for the Down Sampler output."]
        #[inline(always)]
        pub const fn eods_irq_ena(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion interrupt enable for the Down Sampler output."]
        #[inline(always)]
        pub fn set_eods_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of conversion interrupt enable for the decimation filter output."]
        #[inline(always)]
        pub const fn eodf_irq_ena(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of conversion interrupt enable for the decimation filter output."]
        #[inline(always)]
        pub fn set_eodf_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn eos_irq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_eos_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "End of regular sequence interrupt enable."]
        #[inline(always)]
        pub const fn eos_irq_ena(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence interrupt enable."]
        #[inline(always)]
        pub fn set_eos_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn awd_irq(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_awd_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "analog watchdog interrupt enable."]
        #[inline(always)]
        pub const fn awd_irq_ena(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "analog watchdog interrupt enable."]
        #[inline(always)]
        pub fn set_awd_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn ovr_ds_irq(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_ovr_ds_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Down Sampler overrun interrupt enable."]
        #[inline(always)]
        pub const fn ovr_ds_irq_ena(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Down Sampler overrun interrupt enable."]
        #[inline(always)]
        pub fn set_ovr_ds_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "decimation filter overrun interrupt enable."]
        #[inline(always)]
        pub const fn ovr_df_irq_ena(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "decimation filter overrun interrupt enable."]
        #[inline(always)]
        pub fn set_ovr_df_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "decimation filter saturation interrupt enable."]
        #[inline(always)]
        pub const fn df_ovrfl_irq_ena(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "decimation filter saturation interrupt enable."]
        #[inline(always)]
        pub fn set_df_ovrfl_irq_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for IrqEnable {
        #[inline(always)]
        fn default() -> IrqEnable {
            IrqEnable(0)
        }
    }
    impl core::fmt::Debug for IrqEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqEnable")
                .field("eoc_irq", &self.eoc_irq())
                .field("eoc_irq_ena", &self.eoc_irq_ena())
                .field("eods_irq", &self.eods_irq())
                .field("eods_irq_ena", &self.eods_irq_ena())
                .field("eodf_irq_ena", &self.eodf_irq_ena())
                .field("eos_irq", &self.eos_irq())
                .field("eos_irq_ena", &self.eos_irq_ena())
                .field("awd_irq", &self.awd_irq())
                .field("awd_irq_ena", &self.awd_irq_ena())
                .field("ovr_ds_irq", &self.ovr_ds_irq())
                .field("ovr_ds_irq_ena", &self.ovr_ds_irq_ena())
                .field("ovr_df_irq_ena", &self.ovr_df_irq_ena())
                .field("df_ovrfl_irq_ena", &self.df_ovrfl_irq_ena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IrqEnable {{ eoc_irq: {=bool:?}, eoc_irq_ena: {=bool:?}, eods_irq: {=bool:?}, eods_irq_ena: {=bool:?}, eodf_irq_ena: {=bool:?}, eos_irq: {=bool:?}, eos_irq_ena: {=bool:?}, awd_irq: {=bool:?}, awd_irq_ena: {=bool:?}, ovr_ds_irq: {=bool:?}, ovr_ds_irq_ena: {=bool:?}, ovr_df_irq_ena: {=bool:?}, df_ovrfl_irq_ena: {=bool:?} }}" , self . eoc_irq () , self . eoc_irq_ena () , self . eods_irq () , self . eods_irq_ena () , self . eodf_irq_ena () , self . eos_irq () , self . eos_irq_ena () , self . awd_irq () , self . awd_irq_ena () , self . ovr_ds_irq () , self . ovr_ds_irq_ena () , self . ovr_df_irq_ena () , self . df_ovrfl_irq_ena ())
        }
    }
    #[doc = "IRQ_STATUS register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus(pub u32);
    impl IrqStatus {
        #[doc = "EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn eoc_irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EOC_IRQ (Used in test mode only): set when the ADC conversion is completed. When read, provide the status of the interrupt: 0: ADC conversion is not completed 1: ADC conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_eoc_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn eods_irq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EODS_IRQ: set when the Down Sampler conversion is completed. When read, provide the status of the interrupt: 0: Down Sampler conversion is not completed 1: Down Sampler conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_eods_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "set when the decimation filter conversion is completed."]
        #[inline(always)]
        pub const fn eodf_irq(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "set when the decimation filter conversion is completed."]
        #[inline(always)]
        pub fn set_eodf_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn eos_irq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOS_IRQ: set when a sequence of conversion is completed. When read, provide the status of the interrupt: 0: sequence of conversion is not completed 1: sequence of conversion is completed Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_eos_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn awd_irq(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AWD_IRQ: set when an analog watchdog event occurs. When read, provide the status of the interrupt: 0: no analog watchdog event occurred 1: analog watchdog event has occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_awd_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub const fn ovr_ds_irq(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OVR_DS_IRQ: set to indicate a Down Sampler overrun (at least one data is lost) When read, provide the status of the interrupt: 0: no overrun occurred 1: overrun occurred Writing this bit clears the status of the interrupt: 0: no effect 1: clear the interrupt."]
        #[inline(always)]
        pub fn set_ovr_ds_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "set to indicate a decimation filter overrun (a data is lost)."]
        #[inline(always)]
        pub const fn ovr_df_irq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "set to indicate a decimation filter overrun (a data is lost)."]
        #[inline(always)]
        pub fn set_ovr_df_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "set to indicate the decimation filter is saturated."]
        #[inline(always)]
        pub const fn df_ovrfl_irq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set to indicate the decimation filter is saturated."]
        #[inline(always)]
        pub fn set_df_ovrfl_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for IrqStatus {
        #[inline(always)]
        fn default() -> IrqStatus {
            IrqStatus(0)
        }
    }
    impl core::fmt::Debug for IrqStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqStatus")
                .field("eoc_irq", &self.eoc_irq())
                .field("eods_irq", &self.eods_irq())
                .field("eodf_irq", &self.eodf_irq())
                .field("eos_irq", &self.eos_irq())
                .field("awd_irq", &self.awd_irq())
                .field("ovr_ds_irq", &self.ovr_ds_irq())
                .field("ovr_df_irq", &self.ovr_df_irq())
                .field("df_ovrfl_irq", &self.df_ovrfl_irq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IrqStatus {{ eoc_irq: {=bool:?}, eods_irq: {=bool:?}, eodf_irq: {=bool:?}, eos_irq: {=bool:?}, awd_irq: {=bool:?}, ovr_ds_irq: {=bool:?}, ovr_df_irq: {=bool:?}, df_ovrfl_irq: {=bool:?} }}" , self . eoc_irq () , self . eods_irq () , self . eodf_irq () , self . eos_irq () , self . awd_irq () , self . ovr_ds_irq () , self . ovr_df_irq () , self . df_ovrfl_irq ())
        }
    }
    #[doc = "Occasionnal mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OcmCtrl(pub u32);
    impl OcmCtrl {
        #[doc = "select the occasional conversion source."]
        #[inline(always)]
        pub const fn ocm_src(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "select the occasional conversion source."]
        #[inline(always)]
        pub fn set_ocm_src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start occasional conversion in analog audio and full. modes."]
        #[inline(always)]
        pub const fn ocm_ena(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start occasional conversion in analog audio and full. modes."]
        #[inline(always)]
        pub fn set_ocm_ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for OcmCtrl {
        #[inline(always)]
        fn default() -> OcmCtrl {
            OcmCtrl(0)
        }
    }
    impl core::fmt::Debug for OcmCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OcmCtrl")
                .field("ocm_src", &self.ocm_src())
                .field("ocm_ena", &self.ocm_ena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OcmCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OcmCtrl {{ ocm_src: {=bool:?}, ocm_ena: {=bool:?} }}",
                self.ocm_src(),
                self.ocm_ena()
            )
        }
    }
    #[doc = "PGA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PgaConf(pub u32);
    impl PgaConf {
        #[doc = "from 6 to 30 dB."]
        #[inline(always)]
        pub const fn pga_gain(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "from 6 to 30 dB."]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "set the microphone bias voltage."]
        #[inline(always)]
        pub const fn pga_bias(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "set the microphone bias voltage."]
        #[inline(always)]
        pub fn set_pga_bias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for PgaConf {
        #[inline(always)]
        fn default() -> PgaConf {
            PgaConf(0)
        }
    }
    impl core::fmt::Debug for PgaConf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PgaConf")
                .field("pga_gain", &self.pga_gain())
                .field("pga_bias", &self.pga_bias())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PgaConf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PgaConf {{ pga_gain: {=u8:?}, pga_bias: {=u8:?} }}",
                self.pga_gain(),
                self.pga_bias()
            )
        }
    }
    #[doc = "SEQ_1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seq1(pub u32);
    impl Seq1 {
        #[doc = "SEQ0\\[3:0\\]: channel number code for first conversion of the sequence 0000: VINM\\[0\\]
to ADC single negative input 0001: VINM\\[1\\]
to ADC single negative input 0010: VINM\\[2\\]
to ADC single negative input 0011: VINM\\[3\\]
to ADC single negative input 0100: VINP\\[0\\]
to ADC single positive input 0101: VINP\\[1\\]
to ADC single positive input 0110: VINP\\[2\\]
to ADC single positive input 0111: VINP\\[3\\]
to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\]
to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\]
to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\]
to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\]
to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved."]
        #[inline(always)]
        pub const fn seq0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ0\\[3:0\\]: channel number code for first conversion of the sequence 0000: VINM\\[0\\]
to ADC single negative input 0001: VINM\\[1\\]
to ADC single negative input 0010: VINM\\[2\\]
to ADC single negative input 0011: VINM\\[3\\]
to ADC single negative input 0100: VINP\\[0\\]
to ADC single positive input 0101: VINP\\[1\\]
to ADC single positive input 0110: VINP\\[2\\]
to ADC single positive input 0111: VINP\\[3\\]
to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\]
to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\]
to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\]
to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\]
to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved."]
        #[inline(always)]
        pub fn set_seq0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SEQ1\\[3:0\\]: channel number code for second conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ1\\[3:0\\]: channel number code for second conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "SEQ2\\[3:0\\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ2\\[3:0\\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "SEQ3\\[3:0\\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ3\\[3:0\\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "SEQ4\\[3:0\\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ4\\[3:0\\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "SEQ5\\[3:0\\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ5\\[3:0\\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "SEQ6\\[3:0\\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ6\\[3:0\\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "SEQ7\\[3:0\\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ7\\[3:0\\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Seq1 {
        #[inline(always)]
        fn default() -> Seq1 {
            Seq1(0)
        }
    }
    impl core::fmt::Debug for Seq1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seq1")
                .field("seq0", &self.seq0())
                .field("seq1", &self.seq1())
                .field("seq2", &self.seq2())
                .field("seq3", &self.seq3())
                .field("seq4", &self.seq4())
                .field("seq5", &self.seq5())
                .field("seq6", &self.seq6())
                .field("seq7", &self.seq7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seq1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seq1 {{ seq0: {=u8:?}, seq1: {=u8:?}, seq2: {=u8:?}, seq3: {=u8:?}, seq4: {=u8:?}, seq5: {=u8:?}, seq6: {=u8:?}, seq7: {=u8:?} }}" , self . seq0 () , self . seq1 () , self . seq2 () , self . seq3 () , self . seq4 () , self . seq5 () , self . seq6 () , self . seq7 ())
        }
    }
    #[doc = "SEQ_2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seq2(pub u32);
    impl Seq2 {
        #[doc = "SEQ8\\[3:0\\]: channel number code for 9th conversion of the sequence 0000: VINM\\[0\\]
to ADC single negative input 0001: VINM\\[1\\]
to ADC single negative input 0010: VINM\\[2\\]
to ADC single negative input 0011: VINM\\[3\\]
to ADC single negative input 0100: VINP\\[0\\]
to ADC single positive input 0101: VINP\\[1\\]
to ADC single positive input 0110: VINP\\[2\\]
to ADC single positive input 0111: VINP\\[3\\]
to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\]
to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\]
to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\]
to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\]
to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved."]
        #[inline(always)]
        pub const fn seq8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ8\\[3:0\\]: channel number code for 9th conversion of the sequence 0000: VINM\\[0\\]
to ADC single negative input 0001: VINM\\[1\\]
to ADC single negative input 0010: VINM\\[2\\]
to ADC single negative input 0011: VINM\\[3\\]
to ADC single negative input 0100: VINP\\[0\\]
to ADC single positive input 0101: VINP\\[1\\]
to ADC single positive input 0110: VINP\\[2\\]
to ADC single positive input 0111: VINP\\[3\\]
to ADC single positive input 1000: VINP\\[0\\]-VINM\\[0\\]
to ADC differential input 1001: VINP\\[1\\]-VINM\\[1\\]
to ADC differential input 1010: VINP\\[2\\]-VINM\\[2\\]
to ADC differential input 1011: VINP\\[3\\]-VINM\\[3\\]
to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved."]
        #[inline(always)]
        pub fn set_seq8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SEQ9\\[3:0\\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ9\\[3:0\\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "SEQ10\\[3:0\\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ10\\[3:0\\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "SEQ11\\[3:0\\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ11\\[3:0\\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "SEQ12\\[3:0\\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ12\\[3:0\\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "SEQ13\\[3:0\\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ13\\[3:0\\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "SEQ14\\[3:0\\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ14\\[3:0\\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "SEQ15\\[3:0\\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub const fn seq15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "SEQ15\\[3:0\\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail."]
        #[inline(always)]
        pub fn set_seq15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Seq2 {
        #[inline(always)]
        fn default() -> Seq2 {
            Seq2(0)
        }
    }
    impl core::fmt::Debug for Seq2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seq2")
                .field("seq8", &self.seq8())
                .field("seq9", &self.seq9())
                .field("seq10", &self.seq10())
                .field("seq11", &self.seq11())
                .field("seq12", &self.seq12())
                .field("seq13", &self.seq13())
                .field("seq14", &self.seq14())
                .field("seq15", &self.seq15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seq2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seq2 {{ seq8: {=u8:?}, seq9: {=u8:?}, seq10: {=u8:?}, seq11: {=u8:?}, seq12: {=u8:?}, seq13: {=u8:?}, seq14: {=u8:?}, seq15: {=u8:?} }}" , self . seq8 () , self . seq9 () , self . seq10 () , self . seq11 () , self . seq12 () , self . seq13 () , self . seq14 () , self . seq15 ())
        }
    }
    #[doc = "SWITCH register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Switch(pub u32);
    impl Switch {
        #[doc = "SE_VIN_0\\[1:0\\]: input voltage for VINM\\[0\\]
/ VINP\\[0\\]-VINM\\[0\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_0\\[1:0\\]: input voltage for VINM\\[0\\]
/ VINP\\[0\\]-VINM\\[0\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SE_VIN_1\\[1:0\\]: input voltage for VINM\\[1\\]
/ VINP\\[1\\]-VINM\\[1\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_1\\[1:0\\]: input voltage for VINM\\[1\\]
/ VINP\\[1\\]-VINM\\[1\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "SE_VIN_2\\[1:0\\]: input voltage for VINM\\[2\\]
/ VINP\\[2\\]-VINM\\[2\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_2\\[1:0\\]: input voltage for VINM\\[2\\]
/ VINP\\[2\\]-VINM\\[2\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "SE_VIN_3\\[1:0\\]: input voltage for VINM\\[3\\]
/ VINP\\[3\\]-VINM\\[3\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_3\\[1:0\\]: input voltage for VINM\\[3\\]
/ VINP\\[3\\]-VINM\\[3\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "SE_VIN_4\\[1:0\\]: input voltage for VINP\\[0\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_4\\[1:0\\]: input voltage for VINP\\[0\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "SE_VIN_5\\[1:0\\]: input voltage for VINP\\[1\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_5\\[1:0\\]: input voltage for VINP\\[1\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "SE_VIN_6\\[1:0\\]: input voltage for VINP\\[2\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_6\\[1:0\\]: input voltage for VINP\\[2\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "SE_VIN_7\\[1:0\\]: input voltage for VINP\\[3\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub const fn se_vin_7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "SE_VIN_7\\[1:0\\]: input voltage for VINP\\[3\\]
00: Vinput = 1.2V 01: reserved (not used for this cut) 10: Vinput = 2.4V 11: Vinput = 3.6V."]
        #[inline(always)]
        pub fn set_se_vin_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Switch {
        #[inline(always)]
        fn default() -> Switch {
            Switch(0)
        }
    }
    impl core::fmt::Debug for Switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Switch")
                .field("se_vin_0", &self.se_vin_0())
                .field("se_vin_1", &self.se_vin_1())
                .field("se_vin_2", &self.se_vin_2())
                .field("se_vin_3", &self.se_vin_3())
                .field("se_vin_4", &self.se_vin_4())
                .field("se_vin_5", &self.se_vin_5())
                .field("se_vin_6", &self.se_vin_6())
                .field("se_vin_7", &self.se_vin_7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Switch {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Switch {{ se_vin_0: {=u8:?}, se_vin_1: {=u8:?}, se_vin_2: {=u8:?}, se_vin_3: {=u8:?}, se_vin_4: {=u8:?}, se_vin_5: {=u8:?}, se_vin_6: {=u8:?}, se_vin_7: {=u8:?} }}" , self . se_vin_0 () , self . se_vin_1 () , self . se_vin_2 () , self . se_vin_3 () , self . se_vin_4 () , self . se_vin_5 () , self . se_vin_6 () , self . se_vin_7 ())
        }
    }
    #[doc = "Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimerConf(pub u32);
    impl TimerConf {
        #[doc = "define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion."]
        #[inline(always)]
        pub const fn adc_ldo_delay(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion."]
        #[inline(always)]
        pub fn set_adc_ldo_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration."]
        #[inline(always)]
        pub const fn vbias_prech_delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration."]
        #[inline(always)]
        pub fn set_vbias_prech_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer."]
        #[inline(always)]
        pub const fn prech_delay_sel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer."]
        #[inline(always)]
        pub fn set_prech_delay_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for TimerConf {
        #[inline(always)]
        fn default() -> TimerConf {
            TimerConf(0)
        }
    }
    impl core::fmt::Debug for TimerConf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TimerConf")
                .field("adc_ldo_delay", &self.adc_ldo_delay())
                .field("vbias_prech_delay", &self.vbias_prech_delay())
                .field("prech_delay_sel", &self.prech_delay_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimerConf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TimerConf {{ adc_ldo_delay: {=u8:?}, vbias_prech_delay: {=u8:?}, prech_delay_sel: {=bool:?} }}",
                self.adc_ldo_delay(),
                self.vbias_prech_delay(),
                self.prech_delay_sel()
            )
        }
    }
    #[doc = "VERSION_ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VersionId(pub u32);
    impl VersionId {
        #[doc = "VERSION_ID\\[7:0\\]: version of the embedded IP."]
        #[inline(always)]
        pub const fn version_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VERSION_ID\\[7:0\\]: version of the embedded IP."]
        #[inline(always)]
        pub fn set_version_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for VersionId {
        #[inline(always)]
        fn default() -> VersionId {
            VersionId(0)
        }
    }
    impl core::fmt::Debug for VersionId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VersionId")
                .field("version_id", &self.version_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VersionId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VersionId {{ version_id: {=u8:?} }}", self.version_id())
        }
    }
    #[doc = "WD_CONF register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdConf(pub u32);
    impl WdConf {
        #[doc = "AWD_CHX\\[15:0\\]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM\\[0\\]
to ADC negative input Bit1: VINM\\[1\\]
to ADC negative input Bit2: VINM\\[2\\]
to ADC negative input Bit3: VINM\\[3\\]
to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\]
to ADC positive input Bit9: VINP\\[1\\]
to ADC positive input Bit10: VINP\\[2\\]
to ADC positive input Bit11: VINP\\[3\\]
to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input."]
        #[inline(always)]
        pub const fn awd_chx(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "AWD_CHX\\[15:0\\]: analog watchdog channel selection to define which input channel(s) need to be guarded by the watchdog. Bit0: VINM\\[0\\]
to ADC negative input Bit1: VINM\\[1\\]
to ADC negative input Bit2: VINM\\[2\\]
to ADC negative input Bit3: VINM\\[3\\]
to ADC negative input Bit4: Not used Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\\[0\\]
to ADC positive input Bit9: VINP\\[1\\]
to ADC positive input Bit10: VINP\\[2\\]
to ADC positive input Bit11: VINP\\[3\\]
to ADC positive input Bit12: Not used Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input."]
        #[inline(always)]
        pub fn set_awd_chx(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for WdConf {
        #[inline(always)]
        fn default() -> WdConf {
            WdConf(0)
        }
    }
    impl core::fmt::Debug for WdConf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdConf").field("awd_chx", &self.awd_chx()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdConf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdConf {{ awd_chx: {=u16:?} }}", self.awd_chx())
        }
    }
    #[doc = "WD_TH register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdTh(pub u32);
    impl WdTh {
        #[doc = "WD_LT\\[11:0\\]: analog watchdog low level threshold."]
        #[inline(always)]
        pub const fn wd_lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "WD_LT\\[11:0\\]: analog watchdog low level threshold."]
        #[inline(always)]
        pub fn set_wd_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "WD_HT\\[11:0\\]: analog watchdog high level threshold."]
        #[inline(always)]
        pub const fn wd_ht(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "WD_HT\\[11:0\\]: analog watchdog high level threshold."]
        #[inline(always)]
        pub fn set_wd_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for WdTh {
        #[inline(always)]
        fn default() -> WdTh {
            WdTh(0)
        }
    }
    impl core::fmt::Debug for WdTh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdTh")
                .field("wd_lt", &self.wd_lt())
                .field("wd_ht", &self.wd_ht())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdTh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WdTh {{ wd_lt: {=u16:?}, wd_ht: {=u16:?} }}",
                self.wd_lt(),
                self.wd_ht()
            )
        }
    }
}
