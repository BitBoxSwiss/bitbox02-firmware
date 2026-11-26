#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Configuration register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn frcr(self) -> crate::common::Reg<regs::Frcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn slotr(self) -> crate::common::Reg<regs::Slotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Interrupt mask register 2"]
    #[inline(always)]
    pub const fn im(self) -> crate::common::Reg<regs::Im, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Clear flag register"]
    #[inline(always)]
    pub const fn clrfr(self) -> crate::common::Reg<regs::Clrfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
#[doc = "Serial audio interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai {
    ptr: *mut u8,
}
unsafe impl Send for Sai {}
unsafe impl Sync for Sai {}
impl Sai {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Global configuration register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 2usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "PDM control register"]
    #[inline(always)]
    pub const fn pdmcr(self) -> crate::common::Reg<regs::Pdmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "PDM delay register"]
    #[inline(always)]
    pub const fn pdmdly(self) -> crate::common::Reg<regs::Pdmdly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clear flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clrfr(pub u32);
    impl Clrfr {
        #[doc = "Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub const fn covrudr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub fn set_covrudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub const fn cmutedet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub fn set_cmutedet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub const fn cwckcfg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub fn set_cwckcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub const fn ccnrdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub fn set_ccnrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub const fn cafsdet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
        #[inline(always)]
        pub fn set_cafsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
        #[inline(always)]
        pub const fn clfsdet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
        #[inline(always)]
        pub fn set_clfsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Clrfr {
        #[inline(always)]
        fn default() -> Clrfr {
            Clrfr(0)
        }
    }
    impl core::fmt::Debug for Clrfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clrfr")
                .field("covrudr", &self.covrudr())
                .field("cmutedet", &self.cmutedet())
                .field("cwckcfg", &self.cwckcfg())
                .field("ccnrdy", &self.ccnrdy())
                .field("cafsdet", &self.cafsdet())
                .field("clfsdet", &self.clfsdet())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Clrfr {{ covrudr: {=bool:?}, cmutedet: {=bool:?}, cwckcfg: {=bool:?}, ccnrdy: {=bool:?}, cafsdet: {=bool:?}, clfsdet: {=bool:?} }}" , self . covrudr () , self . cmutedet () , self . cwckcfg () , self . ccnrdy () , self . cafsdet () , self . clfsdet ())
        }
    }
    #[doc = "Configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "SAIx audio block mode immediately"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "SAIx audio block mode immediately"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
        #[inline(always)]
        pub const fn prtcfg(&self) -> super::vals::Prtcfg {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Prtcfg::from_bits(val as u8)
        }
        #[doc = "Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
        #[inline(always)]
        pub fn set_prtcfg(&mut self, val: super::vals::Prtcfg) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
        #[inline(always)]
        pub const fn ds(&self) -> super::vals::Ds {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Ds::from_bits(val as u8)
        }
        #[doc = "Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
        #[inline(always)]
        pub fn set_ds(&mut self, val: super::vals::Ds) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
        #[inline(always)]
        pub const fn lsbfirst(&self) -> super::vals::Lsbfirst {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Lsbfirst::from_bits(val as u8)
        }
        #[doc = "Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
        #[inline(always)]
        pub fn set_lsbfirst(&mut self, val: super::vals::Lsbfirst) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
        #[inline(always)]
        pub const fn ckstr(&self) -> super::vals::Ckstr {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Ckstr::from_bits(val as u8)
        }
        #[doc = "Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
        #[inline(always)]
        pub fn set_ckstr(&mut self, val: super::vals::Ckstr) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
        #[inline(always)]
        pub const fn syncen(&self) -> super::vals::Syncen {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Syncen::from_bits(val as u8)
        }
        #[doc = "Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
        #[inline(always)]
        pub fn set_syncen(&mut self, val: super::vals::Syncen) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
        #[inline(always)]
        pub const fn mono(&self) -> super::vals::Mono {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Mono::from_bits(val as u8)
        }
        #[doc = "Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
        #[inline(always)]
        pub fn set_mono(&mut self, val: super::vals::Mono) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
        #[inline(always)]
        pub const fn outdriv(&self) -> super::vals::Outdriv {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Outdriv::from_bits(val as u8)
        }
        #[doc = "Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
        #[inline(always)]
        pub fn set_outdriv(&mut self, val: super::vals::Outdriv) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
        #[inline(always)]
        pub const fn saien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
        #[inline(always)]
        pub fn set_saien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No fixed divider between MCLK and FS"]
        #[inline(always)]
        pub const fn nodiv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No fixed divider between MCLK and FS"]
        #[inline(always)]
        pub fn set_nodiv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
        #[inline(always)]
        pub const fn mckdiv(&self) -> super::vals::Mckdiv {
            let val = (self.0 >> 20usize) & 0x3f;
            super::vals::Mckdiv::from_bits(val as u8)
        }
        #[doc = "Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
        #[inline(always)]
        pub fn set_mckdiv(&mut self, val: super::vals::Mckdiv) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val.to_bits() as u32) & 0x3f) << 20usize);
        }
        #[doc = "Oversampling ratio for master clock"]
        #[inline(always)]
        pub const fn osr(&self) -> super::vals::Osr {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Osr::from_bits(val as u8)
        }
        #[doc = "Oversampling ratio for master clock"]
        #[inline(always)]
        pub fn set_osr(&mut self, val: super::vals::Osr) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "Master clock generation enable"]
        #[inline(always)]
        pub const fn mcken(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock generation enable"]
        #[inline(always)]
        pub fn set_mcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("mode", &self.mode())
                .field("prtcfg", &self.prtcfg())
                .field("ds", &self.ds())
                .field("lsbfirst", &self.lsbfirst())
                .field("ckstr", &self.ckstr())
                .field("syncen", &self.syncen())
                .field("mono", &self.mono())
                .field("outdriv", &self.outdriv())
                .field("saien", &self.saien())
                .field("dmaen", &self.dmaen())
                .field("nodiv", &self.nodiv())
                .field("mckdiv", &self.mckdiv())
                .field("osr", &self.osr())
                .field("mcken", &self.mcken())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr1 {{ mode: {:?}, prtcfg: {:?}, ds: {:?}, lsbfirst: {:?}, ckstr: {:?}, syncen: {:?}, mono: {:?}, outdriv: {:?}, saien: {=bool:?}, dmaen: {=bool:?}, nodiv: {=bool:?}, mckdiv: {:?}, osr: {:?}, mcken: {=bool:?} }}" , self . mode () , self . prtcfg () , self . ds () , self . lsbfirst () , self . ckstr () , self . syncen () , self . mono () , self . outdriv () , self . saien () , self . dmaen () , self . nodiv () , self . mckdiv () , self . osr () , self . mcken ())
        }
    }
    #[doc = "Configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "FIFO threshold. This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn fth(&self) -> super::vals::Fth {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Fth::from_bits(val as u8)
        }
        #[doc = "FIFO threshold. This bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_fth(&mut self, val: super::vals::Fth) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
        #[inline(always)]
        pub const fn fflush(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
        #[inline(always)]
        pub fn set_fflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
        #[inline(always)]
        pub const fn tris(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
        #[inline(always)]
        pub fn set_tris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
        #[inline(always)]
        pub const fn mute(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
        #[inline(always)]
        pub fn set_mute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
        #[inline(always)]
        pub const fn muteval(&self) -> super::vals::Muteval {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Muteval::from_bits(val as u8)
        }
        #[doc = "Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
        #[inline(always)]
        pub fn set_muteval(&mut self, val: super::vals::Muteval) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
        #[inline(always)]
        pub const fn mutecnt(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x3f;
            val as u8
        }
        #[doc = "Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
        #[inline(always)]
        pub fn set_mutecnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 7usize)) | (((val as u32) & 0x3f) << 7usize);
        }
        #[doc = "Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
        #[inline(always)]
        pub const fn cpl(&self) -> super::vals::Cpl {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Cpl::from_bits(val as u8)
        }
        #[doc = "Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
        #[inline(always)]
        pub fn set_cpl(&mut self, val: super::vals::Cpl) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Comp {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Comp::from_bits(val as u8)
        }
        #[doc = "Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
        #[inline(always)]
        pub fn set_comp(&mut self, val: super::vals::Comp) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("fth", &self.fth())
                .field("fflush", &self.fflush())
                .field("tris", &self.tris())
                .field("mute", &self.mute())
                .field("muteval", &self.muteval())
                .field("mutecnt", &self.mutecnt())
                .field("cpl", &self.cpl())
                .field("comp", &self.comp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr2 {{ fth: {:?}, fflush: {=bool:?}, tris: {=bool:?}, mute: {=bool:?}, muteval: {:?}, mutecnt: {=u8:?}, cpl: {:?}, comp: {:?} }}" , self . fth () , self . fflush () , self . tris () , self . mute () , self . muteval () , self . mutecnt () , self . cpl () , self . comp ())
        }
    }
    #[doc = "Data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
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
    #[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frcr(pub u32);
    impl Frcr {
        #[doc = "Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\\[7:0\\]
+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\\[4:0\\]
of SAI_xSLOTR register (NBSLOT\\[3:0\\]
= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration."]
        #[inline(always)]
        pub const fn frl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\\[7:0\\]
+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\\[4:0\\]
of SAI_xSLOTR register (NBSLOT\\[3:0\\]
= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration."]
        #[inline(always)]
        pub fn set_frl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\\[6:0\\]
+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
        #[inline(always)]
        pub const fn fsall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\\[6:0\\]
+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
        #[inline(always)]
        pub fn set_fsall(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled."]
        #[inline(always)]
        pub const fn fsdef(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled."]
        #[inline(always)]
        pub fn set_fsdef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
        #[inline(always)]
        pub const fn fspol(&self) -> super::vals::Fspol {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Fspol::from_bits(val as u8)
        }
        #[doc = "Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
        #[inline(always)]
        pub fn set_fspol(&mut self, val: super::vals::Fspol) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
        #[inline(always)]
        pub const fn fsoff(&self) -> super::vals::Fsoff {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Fsoff::from_bits(val as u8)
        }
        #[doc = "Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
        #[inline(always)]
        pub fn set_fsoff(&mut self, val: super::vals::Fsoff) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Frcr {
        #[inline(always)]
        fn default() -> Frcr {
            Frcr(0)
        }
    }
    impl core::fmt::Debug for Frcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frcr")
                .field("frl", &self.frl())
                .field("fsall", &self.fsall())
                .field("fsdef", &self.fsdef())
                .field("fspol", &self.fspol())
                .field("fsoff", &self.fsoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Frcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Frcr {{ frl: {=u8:?}, fsall: {=u8:?}, fsdef: {=bool:?}, fspol: {:?}, fsoff: {:?} }}",
                self.frl(),
                self.fsall(),
                self.fsdef(),
                self.fspol(),
                self.fsoff()
            )
        }
    }
    #[doc = "Global configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "Synchronization inputs"]
        #[inline(always)]
        pub const fn syncin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization inputs"]
        #[inline(always)]
        pub fn set_syncin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Synchronization outputs These bits are set and cleared by software."]
        #[inline(always)]
        pub const fn syncout(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization outputs These bits are set and cleared by software."]
        #[inline(always)]
        pub fn set_syncout(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("syncin", &self.syncin())
                .field("syncout", &self.syncout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gcr {{ syncin: {=u8:?}, syncout: {=u8:?} }}",
                self.syncin(),
                self.syncout()
            )
        }
    }
    #[doc = "Interrupt mask register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Im(pub u32);
    impl Im {
        #[doc = "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
        #[inline(always)]
        pub const fn ovrudrie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun/underrun interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set."]
        #[inline(always)]
        pub fn set_ovrudrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
        #[inline(always)]
        pub const fn mutedetie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mute detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set. This bit has a meaning only if the audio block is configured in receiver mode."]
        #[inline(always)]
        pub fn set_mutedetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in TDM mode and is meaningless in other modes."]
        #[inline(always)]
        pub const fn wckcfgie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wrong clock configuration interrupt enable. This bit is set and cleared by software. This bit is taken into account only if the audio block is configured as a master (MODE\\[1\\]
= 0) and NODIV = 0. It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set. Note: This bit is used only in TDM mode and is meaningless in other modes."]
        #[inline(always)]
        pub fn set_wckcfgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interruption in receiver mode,"]
        #[inline(always)]
        pub const fn freqie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO request interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set. Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be configured before setting FREQIE to avoid a parasitic interruption in receiver mode,"]
        #[inline(always)]
        pub fn set_freqie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Codec not ready interrupt enable (AC97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interruption i generated. This bit has a meaning only if the AC97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
        #[inline(always)]
        pub const fn cnrdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Codec not ready interrupt enable (AC97). This bit is set and cleared by software. When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC97 frame if the Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR register is set and an interruption i generated. This bit has a meaning only if the AC97 mode is selected through PRTCFG\\[1:0\\]
bits and the audio block is operates as a receiver."]
        #[inline(always)]
        pub fn set_cnrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
        #[inline(always)]
        pub const fn afsdetie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the AFSDET bit in the SAI_xSR register is set. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
        #[inline(always)]
        pub fn set_afsdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
        #[inline(always)]
        pub const fn lfsdetie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Late frame synchronization detection interrupt enable. This bit is set and cleared by software. When this bit is set, an interrupt will be generated if the LFSDET bit is set in the SAI_xSR register. This bit is meaningless in AC97, SPDIF mode or when the audio block operates as a master."]
        #[inline(always)]
        pub fn set_lfsdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Im {
        #[inline(always)]
        fn default() -> Im {
            Im(0)
        }
    }
    impl core::fmt::Debug for Im {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Im")
                .field("ovrudrie", &self.ovrudrie())
                .field("mutedetie", &self.mutedetie())
                .field("wckcfgie", &self.wckcfgie())
                .field("freqie", &self.freqie())
                .field("cnrdyie", &self.cnrdyie())
                .field("afsdetie", &self.afsdetie())
                .field("lfsdetie", &self.lfsdetie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Im {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Im {{ ovrudrie: {=bool:?}, mutedetie: {=bool:?}, wckcfgie: {=bool:?}, freqie: {=bool:?}, cnrdyie: {=bool:?}, afsdetie: {=bool:?}, lfsdetie: {=bool:?} }}" , self . ovrudrie () , self . mutedetie () , self . wckcfgie () , self . freqie () , self . cnrdyie () , self . afsdetie () , self . lfsdetie ())
        }
    }
    #[doc = "PDM control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdmcr(pub u32);
    impl Pdmcr {
        #[doc = "PDM enable"]
        #[inline(always)]
        pub const fn pdmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PDM enable"]
        #[inline(always)]
        pub fn set_pdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Number of microphones"]
        #[inline(always)]
        pub const fn micnbr(&self) -> super::vals::Micnbr {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Micnbr::from_bits(val as u8)
        }
        #[doc = "Number of microphones"]
        #[inline(always)]
        pub fn set_micnbr(&mut self, val: super::vals::Micnbr) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Clock enable of bitstream clock number 1"]
        #[inline(always)]
        pub const fn cken(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clock enable of bitstream clock number 1"]
        #[inline(always)]
        pub fn set_cken(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pdmcr {
        #[inline(always)]
        fn default() -> Pdmcr {
            Pdmcr(0)
        }
    }
    impl core::fmt::Debug for Pdmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdmcr")
                .field("pdmen", &self.pdmen())
                .field("micnbr", &self.micnbr())
                .field("cken[0]", &self.cken(0usize))
                .field("cken[1]", &self.cken(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdmcr {{ pdmen: {=bool:?}, micnbr: {:?}, cken[0]: {=bool:?}, cken[1]: {=bool:?} }}",
                self.pdmen(),
                self.micnbr(),
                self.cken(0usize),
                self.cken(1usize)
            )
        }
    }
    #[doc = "PDM delay register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdmdly(pub u32);
    impl Pdmdly {
        #[doc = "Delay line adjust for first microphone of pair 1"]
        #[inline(always)]
        pub const fn dlyml(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Delay line adjust for first microphone of pair 1"]
        #[inline(always)]
        pub fn set_dlyml(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
        #[doc = "Delay line adjust for second microphone of pair 1"]
        #[inline(always)]
        pub const fn dlymr(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Delay line adjust for second microphone of pair 1"]
        #[inline(always)]
        pub fn set_dlymr(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Pdmdly {
        #[inline(always)]
        fn default() -> Pdmdly {
            Pdmdly(0)
        }
    }
    impl core::fmt::Debug for Pdmdly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdmdly")
                .field("dlyml[0]", &self.dlyml(0usize))
                .field("dlyml[1]", &self.dlyml(1usize))
                .field("dlyml[2]", &self.dlyml(2usize))
                .field("dlyml[3]", &self.dlyml(3usize))
                .field("dlymr[0]", &self.dlymr(0usize))
                .field("dlymr[1]", &self.dlymr(1usize))
                .field("dlymr[2]", &self.dlymr(2usize))
                .field("dlymr[3]", &self.dlymr(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdmdly {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pdmdly {{ dlyml[0]: {=u8:?}, dlyml[1]: {=u8:?}, dlyml[2]: {=u8:?}, dlyml[3]: {=u8:?}, dlymr[0]: {=u8:?}, dlymr[1]: {=u8:?}, dlymr[2]: {=u8:?}, dlymr[3]: {=u8:?} }}" , self . dlyml (0usize) , self . dlyml (1usize) , self . dlyml (2usize) , self . dlyml (3usize) , self . dlymr (0usize) , self . dlymr (1usize) , self . dlymr (2usize) , self . dlymr (3usize))
        }
    }
    #[doc = "This register has no meaning in AC97 and SPDIF audio protocol"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slotr(pub u32);
    impl Slotr {
        #[doc = "First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub const fn fboff(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub fn set_fboff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI will be undetermined. Refer to Section: Output data line management on an inactive slot for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub const fn slotsz(&self) -> super::vals::Slotsz {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Slotsz::from_bits(val as u8)
        }
        #[doc = "Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI will be undetermined. Refer to Section: Output data line management on an inactive slot for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub fn set_slotsz(&mut self, val: super::vals::Slotsz) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub const fn nbslot(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub fn set_nbslot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub const fn sloten(&self) -> super::vals::Sloten {
            let val = (self.0 >> 16usize) & 0xffff;
            super::vals::Sloten::from_bits(val as u16)
        }
        #[doc = "Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC97 or SPDIF mode."]
        #[inline(always)]
        pub fn set_sloten(&mut self, val: super::vals::Sloten) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Slotr {
        #[inline(always)]
        fn default() -> Slotr {
            Slotr(0)
        }
    }
    impl core::fmt::Debug for Slotr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slotr")
                .field("fboff", &self.fboff())
                .field("slotsz", &self.slotsz())
                .field("nbslot", &self.nbslot())
                .field("sloten", &self.sloten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Slotr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Slotr {{ fboff: {=u8:?}, slotsz: {:?}, nbslot: {=u8:?}, sloten: {:?} }}",
                self.fboff(),
                self.slotsz(),
                self.nbslot(),
                self.sloten()
            )
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub const fn ovrudr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub fn set_ovrudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
        #[inline(always)]
        pub const fn mutedet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
        #[inline(always)]
        pub fn set_mutedet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub const fn wckcfg(&self) -> super::vals::Wckcfg {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wckcfg::from_bits(val as u8)
        }
        #[doc = "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub fn set_wckcfg(&mut self, val: super::vals::Wckcfg) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
        #[inline(always)]
        pub const fn freq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
        #[inline(always)]
        pub fn set_freq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub const fn cnrdy(&self) -> super::vals::Cnrdy {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Cnrdy::from_bits(val as u8)
        }
        #[doc = "Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub fn set_cnrdy(&mut self, val: super::vals::Cnrdy) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub const fn afsdet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
        #[inline(always)]
        pub fn set_afsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
        #[inline(always)]
        pub const fn lfsdet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
        #[inline(always)]
        pub fn set_lfsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:"]
        #[inline(always)]
        pub const fn flvl(&self) -> super::vals::Flvl {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Flvl::from_bits(val as u8)
        }
        #[doc = "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:"]
        #[inline(always)]
        pub fn set_flvl(&mut self, val: super::vals::Flvl) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
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
                .field("ovrudr", &self.ovrudr())
                .field("mutedet", &self.mutedet())
                .field("wckcfg", &self.wckcfg())
                .field("freq", &self.freq())
                .field("cnrdy", &self.cnrdy())
                .field("afsdet", &self.afsdet())
                .field("lfsdet", &self.lfsdet())
                .field("flvl", &self.flvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ ovrudr: {=bool:?}, mutedet: {=bool:?}, wckcfg: {:?}, freq: {=bool:?}, cnrdy: {:?}, afsdet: {=bool:?}, lfsdet: {=bool:?}, flvl: {:?} }}" , self . ovrudr () , self . mutedet () , self . wckcfg () , self . freq () , self . cnrdy () , self . afsdet () , self . lfsdet () , self . flvl ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckstr {
        #[doc = "Data strobing edge is falling edge of SCK"]
        FALLING_EDGE = 0x0,
        #[doc = "Data strobing edge is rising edge of SCK"]
        RISING_EDGE = 0x01,
    }
    impl Ckstr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckstr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckstr {
        #[inline(always)]
        fn from(val: u8) -> Ckstr {
            Ckstr::from_bits(val)
        }
    }
    impl From<Ckstr> for u8 {
        #[inline(always)]
        fn from(val: Ckstr) -> u8 {
            Ckstr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cnrdy {
        #[doc = "External AC'97 Codec is ready"]
        READY = 0x0,
        #[doc = "External AC'97 Codec is not ready"]
        NOT_READY = 0x01,
    }
    impl Cnrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cnrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cnrdy {
        #[inline(always)]
        fn from(val: u8) -> Cnrdy {
            Cnrdy::from_bits(val)
        }
    }
    impl From<Cnrdy> for u8 {
        #[inline(always)]
        fn from(val: Cnrdy) -> u8 {
            Cnrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Comp {
        #[doc = "No companding algorithm"]
        NO_COMPANDING = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "μ-Law algorithm"]
        MU_LAW = 0x02,
        #[doc = "A-Law algorithm"]
        ALAW = 0x03,
    }
    impl Comp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Comp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Comp {
        #[inline(always)]
        fn from(val: u8) -> Comp {
            Comp::from_bits(val)
        }
    }
    impl From<Comp> for u8 {
        #[inline(always)]
        fn from(val: Comp) -> u8 {
            Comp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpl {
        #[doc = "1's complement representation"]
        ONES_COMPLEMENT = 0x0,
        #[doc = "2's complement representation"]
        TWOS_COMPLEMENT = 0x01,
    }
    impl Cpl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpl {
        #[inline(always)]
        fn from(val: u8) -> Cpl {
            Cpl::from_bits(val)
        }
    }
    impl From<Cpl> for u8 {
        #[inline(always)]
        fn from(val: Cpl) -> u8 {
            Cpl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ds {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "8 bits"]
        BIT8 = 0x02,
        #[doc = "10 bits"]
        BIT10 = 0x03,
        #[doc = "16 bits"]
        BIT16 = 0x04,
        #[doc = "20 bits"]
        BIT20 = 0x05,
        #[doc = "24 bits"]
        BIT24 = 0x06,
        #[doc = "32 bits"]
        BIT32 = 0x07,
    }
    impl Ds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ds {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ds {
        #[inline(always)]
        fn from(val: u8) -> Ds {
            Ds::from_bits(val)
        }
    }
    impl From<Ds> for u8 {
        #[inline(always)]
        fn from(val: Ds) -> u8 {
            Ds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Flvl {
        #[doc = "FIFO empty"]
        EMPTY = 0x0,
        #[doc = "FIFO <= 1/4 but not empty"]
        QUARTER1 = 0x01,
        #[doc = "1/4 < FIFO <= 1/2"]
        QUARTER2 = 0x02,
        #[doc = "1/2 < FIFO <= 3/4"]
        QUARTER3 = 0x03,
        #[doc = "3/4 < FIFO but not full"]
        QUARTER4 = 0x04,
        #[doc = "FIFO full"]
        FULL = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Flvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Flvl {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Flvl {
        #[inline(always)]
        fn from(val: u8) -> Flvl {
            Flvl::from_bits(val)
        }
    }
    impl From<Flvl> for u8 {
        #[inline(always)]
        fn from(val: Flvl) -> u8 {
            Flvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fsoff {
        #[doc = "FS is asserted on the first bit of the slot 0"]
        ON_FIRST = 0x0,
        #[doc = "FS is asserted one bit before the first bit of the slot 0"]
        BEFORE_FIRST = 0x01,
    }
    impl Fsoff {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fsoff {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fsoff {
        #[inline(always)]
        fn from(val: u8) -> Fsoff {
            Fsoff::from_bits(val)
        }
    }
    impl From<Fsoff> for u8 {
        #[inline(always)]
        fn from(val: Fsoff) -> u8 {
            Fsoff::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fspol {
        #[doc = "FS is active low (falling edge)"]
        FALLING_EDGE = 0x0,
        #[doc = "FS is active high (rising edge)"]
        RISING_EDGE = 0x01,
    }
    impl Fspol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fspol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fspol {
        #[inline(always)]
        fn from(val: u8) -> Fspol {
            Fspol::from_bits(val)
        }
    }
    impl From<Fspol> for u8 {
        #[inline(always)]
        fn from(val: Fspol) -> u8 {
            Fspol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fth {
        #[doc = "FIFO empty"]
        EMPTY = 0x0,
        #[doc = "1/4 FIFO"]
        QUARTER1 = 0x01,
        #[doc = "1/2 FIFO"]
        QUARTER2 = 0x02,
        #[doc = "3/4 FIFO"]
        QUARTER3 = 0x03,
        #[doc = "FIFO full"]
        FULL = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fth {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fth {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fth {
        #[inline(always)]
        fn from(val: u8) -> Fth {
            Fth::from_bits(val)
        }
    }
    impl From<Fth> for u8 {
        #[inline(always)]
        fn from(val: Fth) -> u8 {
            Fth::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsbfirst {
        #[doc = "Data are transferred with MSB first"]
        MSB_FIRST = 0x0,
        #[doc = "Data are transferred with LSB first"]
        LSB_FIRST = 0x01,
    }
    impl Lsbfirst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsbfirst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsbfirst {
        #[inline(always)]
        fn from(val: u8) -> Lsbfirst {
            Lsbfirst::from_bits(val)
        }
    }
    impl From<Lsbfirst> for u8 {
        #[inline(always)]
        fn from(val: Lsbfirst) -> u8 {
            Lsbfirst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mckdiv {
        _RESERVED_0 = 0x0,
        DIV1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
        DIV8 = 0x08,
        DIV9 = 0x09,
        DIV10 = 0x0a,
        DIV11 = 0x0b,
        DIV12 = 0x0c,
        DIV13 = 0x0d,
        DIV14 = 0x0e,
        DIV15 = 0x0f,
        DIV16 = 0x10,
        DIV17 = 0x11,
        DIV18 = 0x12,
        DIV19 = 0x13,
        DIV20 = 0x14,
        DIV21 = 0x15,
        DIV22 = 0x16,
        DIV23 = 0x17,
        DIV24 = 0x18,
        DIV25 = 0x19,
        DIV26 = 0x1a,
        DIV27 = 0x1b,
        DIV28 = 0x1c,
        DIV29 = 0x1d,
        DIV30 = 0x1e,
        DIV31 = 0x1f,
        DIV32 = 0x20,
        DIV33 = 0x21,
        DIV34 = 0x22,
        DIV35 = 0x23,
        DIV36 = 0x24,
        DIV37 = 0x25,
        DIV38 = 0x26,
        DIV39 = 0x27,
        DIV40 = 0x28,
        DIV41 = 0x29,
        DIV42 = 0x2a,
        DIV43 = 0x2b,
        DIV44 = 0x2c,
        DIV45 = 0x2d,
        DIV46 = 0x2e,
        DIV47 = 0x2f,
        DIV48 = 0x30,
        DIV49 = 0x31,
        DIV50 = 0x32,
        DIV51 = 0x33,
        DIV52 = 0x34,
        DIV53 = 0x35,
        DIV54 = 0x36,
        DIV55 = 0x37,
        DIV56 = 0x38,
        DIV57 = 0x39,
        DIV58 = 0x3a,
        DIV59 = 0x3b,
        DIV60 = 0x3c,
        DIV61 = 0x3d,
        DIV62 = 0x3e,
        DIV63 = 0x3f,
    }
    impl Mckdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mckdiv {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mckdiv {
        #[inline(always)]
        fn from(val: u8) -> Mckdiv {
            Mckdiv::from_bits(val)
        }
    }
    impl From<Mckdiv> for u8 {
        #[inline(always)]
        fn from(val: Mckdiv) -> u8 {
            Mckdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Micnbr {
        #[doc = "Configuration with 2 microphones"]
        _2MICS = 0x0,
        #[doc = "Configuration with 4 microphones"]
        _4MICS = 0x01,
        #[doc = "Configuration with 6 microphones"]
        _6MICS = 0x02,
        #[doc = "Configuration with 8 microphones"]
        _8MICS = 0x03,
    }
    impl Micnbr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Micnbr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Micnbr {
        #[inline(always)]
        fn from(val: u8) -> Micnbr {
            Micnbr::from_bits(val)
        }
    }
    impl From<Micnbr> for u8 {
        #[inline(always)]
        fn from(val: Micnbr) -> u8 {
            Micnbr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Master transmitter"]
        MASTER_TX = 0x0,
        #[doc = "Master receiver"]
        MASTER_RX = 0x01,
        #[doc = "Slave transmitter"]
        SLAVE_TX = 0x02,
        #[doc = "Slave receiver"]
        SLAVE_RX = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Mono {
        #[doc = "Stereo mode"]
        STEREO = 0x0,
        #[doc = "Mono mode"]
        MONO = 0x01,
    }
    impl Mono {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mono {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mono {
        #[inline(always)]
        fn from(val: u8) -> Mono {
            Mono::from_bits(val)
        }
    }
    impl From<Mono> for u8 {
        #[inline(always)]
        fn from(val: Mono) -> u8 {
            Mono::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Muteval {
        #[doc = "Bit value 0 is sent during the mute mode"]
        SEND_ZERO = 0x0,
        #[doc = "Last values are sent during the mute mode"]
        SEND_LAST = 0x01,
    }
    impl Muteval {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Muteval {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Muteval {
        #[inline(always)]
        fn from(val: u8) -> Muteval {
            Muteval::from_bits(val)
        }
    }
    impl From<Muteval> for u8 {
        #[inline(always)]
        fn from(val: Muteval) -> u8 {
            Muteval::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Osr {
        #[doc = "Master clock frequency = Fless thansub FSless than/sub x 256"]
        LESS_THAN256 = 0x0,
        #[doc = "Master clock frequency = Fless thansub FSless than/sub x 512"]
        LESS_THAN512 = 0x01,
    }
    impl Osr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Osr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Osr {
        #[inline(always)]
        fn from(val: u8) -> Osr {
            Osr::from_bits(val)
        }
    }
    impl From<Osr> for u8 {
        #[inline(always)]
        fn from(val: Osr) -> u8 {
            Osr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Outdriv {
        #[doc = "Audio block output driven when SAIEN is set"]
        ON_START = 0x0,
        #[doc = "Audio block output driven immediately after the setting of this bit"]
        IMMEDIATELY = 0x01,
    }
    impl Outdriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Outdriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Outdriv {
        #[inline(always)]
        fn from(val: u8) -> Outdriv {
            Outdriv::from_bits(val)
        }
    }
    impl From<Outdriv> for u8 {
        #[inline(always)]
        fn from(val: Outdriv) -> u8 {
            Outdriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Prtcfg {
        #[doc = "Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
        FREE = 0x0,
        #[doc = "SPDIF protocol"]
        SPDIF = 0x01,
        #[doc = "AC'97 protocol"]
        AC97 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Prtcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prtcfg {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prtcfg {
        #[inline(always)]
        fn from(val: u8) -> Prtcfg {
            Prtcfg::from_bits(val)
        }
    }
    impl From<Prtcfg> for u8 {
        #[inline(always)]
        fn from(val: Prtcfg) -> u8 {
            Prtcfg::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sloten(u16);
    impl Sloten {
        #[doc = "Inactive slot"]
        pub const INACTIVE: Self = Self(0x0);
        #[doc = "Active slot"]
        pub const ACTIVE: Self = Self(0x01);
    }
    impl Sloten {
        pub const fn from_bits(val: u16) -> Sloten {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Sloten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("INACTIVE"),
                0x01 => f.write_str("ACTIVE"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sloten {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "INACTIVE"),
                0x01 => defmt::write!(f, "ACTIVE"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Sloten {
        #[inline(always)]
        fn from(val: u16) -> Sloten {
            Sloten::from_bits(val)
        }
    }
    impl From<Sloten> for u16 {
        #[inline(always)]
        fn from(val: Sloten) -> u16 {
            Sloten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Slotsz {
        #[doc = "The slot size is equivalent to the data size (specified in DS\\[3:0\\]
in the SAI_xCR1 register)"]
        DATA_SIZE = 0x0,
        #[doc = "16-bit"]
        BIT16 = 0x01,
        #[doc = "32-bit"]
        BIT32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Slotsz {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Slotsz {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Slotsz {
        #[inline(always)]
        fn from(val: u8) -> Slotsz {
            Slotsz::from_bits(val)
        }
    }
    impl From<Slotsz> for u8 {
        #[inline(always)]
        fn from(val: Slotsz) -> u8 {
            Slotsz::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syncen {
        #[doc = "audio sub-block in asynchronous mode"]
        ASYNCHRONOUS = 0x0,
        #[doc = "audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
        INTERNAL = 0x01,
        #[doc = "audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
        EXTERNAL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Syncen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncen {
        #[inline(always)]
        fn from(val: u8) -> Syncen {
            Syncen::from_bits(val)
        }
    }
    impl From<Syncen> for u8 {
        #[inline(always)]
        fn from(val: Syncen) -> u8 {
            Syncen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wckcfg {
        #[doc = "Clock configuration is correct"]
        CORRECT = 0x0,
        #[doc = "Clock configuration does not respect the rule concerning the frame length specification"]
        WRONG = 0x01,
    }
    impl Wckcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wckcfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wckcfg {
        #[inline(always)]
        fn from(val: u8) -> Wckcfg {
            Wckcfg::from_bits(val)
        }
    }
    impl From<Wckcfg> for u8 {
        #[inline(always)]
        fn from(val: Wckcfg) -> u8 {
            Wckcfg::to_bits(val)
        }
    }
}
