#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Alternate function I/O"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afio {
    ptr: *mut u8,
}
unsafe impl Send for Afio {}
unsafe impl Sync for Afio {}
impl Afio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Event Control Register (AFIO_EVCR)"]
    #[inline(always)]
    pub const fn evcr(self) -> crate::common::Reg<regs::Evcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
    #[inline(always)]
    pub const fn mapr(self) -> crate::common::Reg<regs::Mapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register"]
    #[inline(always)]
    pub const fn mapr2(self) -> crate::common::Reg<regs::Mapr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Event Control Register (AFIO_EVCR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evcr(pub u32);
    impl Evcr {
        #[doc = "Pin selection"]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pin selection"]
        #[inline(always)]
        pub fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Port selection"]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Port selection"]
        #[inline(always)]
        pub fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Event Output Enable"]
        #[inline(always)]
        pub const fn evoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Event Output Enable"]
        #[inline(always)]
        pub fn set_evoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Evcr {
        #[inline(always)]
        fn default() -> Evcr {
            Evcr(0)
        }
    }
    impl core::fmt::Debug for Evcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evcr")
                .field("pin", &self.pin())
                .field("port", &self.port())
                .field("evoe", &self.evoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Evcr {{ pin: {=u8:?}, port: {=u8:?}, evoe: {=bool:?} }}",
                self.pin(),
                self.port(),
                self.evoe()
            )
        }
    }
    #[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI12 configuration"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mapr(pub u32);
    impl Mapr {
        #[doc = "SPI1 remapping"]
        #[inline(always)]
        pub const fn spi1_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 remapping"]
        #[inline(always)]
        pub fn set_spi1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C1 remapping"]
        #[inline(always)]
        pub const fn i2c1_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 remapping"]
        #[inline(always)]
        pub fn set_i2c1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 remapping"]
        #[inline(always)]
        pub const fn usart1_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 remapping"]
        #[inline(always)]
        pub fn set_usart1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART2 remapping"]
        #[inline(always)]
        pub const fn usart2_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 remapping"]
        #[inline(always)]
        pub fn set_usart2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "USART3 remapping"]
        #[inline(always)]
        pub const fn usart3_remap(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USART3 remapping"]
        #[inline(always)]
        pub fn set_usart3_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "TIM1 remapping"]
        #[inline(always)]
        pub const fn tim1_remap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TIM1 remapping"]
        #[inline(always)]
        pub fn set_tim1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "TIM2 remapping"]
        #[inline(always)]
        pub const fn tim2_remap(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "TIM2 remapping"]
        #[inline(always)]
        pub fn set_tim2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "TIM3 remapping"]
        #[inline(always)]
        pub const fn tim3_remap(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "TIM3 remapping"]
        #[inline(always)]
        pub fn set_tim3_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TIM4 remapping"]
        #[inline(always)]
        pub const fn tim4_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 remapping"]
        #[inline(always)]
        pub fn set_tim4_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CAN1 remapping"]
        #[inline(always)]
        pub const fn can1_remap(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "CAN1 remapping"]
        #[inline(always)]
        pub fn set_can1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT"]
        #[inline(always)]
        pub const fn pd01_remap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT"]
        #[inline(always)]
        pub fn set_pd01_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Set and cleared by software"]
        #[inline(always)]
        pub const fn tim5ch4_iremap(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Set and cleared by software"]
        #[inline(always)]
        pub fn set_tim5ch4_iremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC 1 External trigger injected conversion remapping"]
        #[inline(always)]
        pub const fn adc1_etrginj_remap(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 External trigger injected conversion remapping"]
        #[inline(always)]
        pub fn set_adc1_etrginj_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC 1 external trigger regular conversion remapping"]
        #[inline(always)]
        pub const fn adc1_etrgreg_remap(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 external trigger regular conversion remapping"]
        #[inline(always)]
        pub fn set_adc1_etrgreg_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC 2 external trigger injected conversion remapping"]
        #[inline(always)]
        pub const fn adc2_etrginj_remap(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 external trigger injected conversion remapping"]
        #[inline(always)]
        pub fn set_adc2_etrginj_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ADC 2 external trigger regular conversion remapping"]
        #[inline(always)]
        pub const fn adc2_etrgreg_remap(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 external trigger regular conversion remapping"]
        #[inline(always)]
        pub fn set_adc2_etrgreg_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Ethernet MAC I/O remapping"]
        #[inline(always)]
        pub const fn eth_remap(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC I/O remapping"]
        #[inline(always)]
        pub fn set_eth_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CAN2 I/O remapping"]
        #[inline(always)]
        pub const fn can2_remap(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 I/O remapping"]
        #[inline(always)]
        pub fn set_can2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MII or RMII selection"]
        #[inline(always)]
        pub const fn mii_rmii_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MII or RMII selection"]
        #[inline(always)]
        pub fn set_mii_rmii_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Serial wire JTAG configuration (must be set to NoOp to leave it unchanged!)"]
        #[inline(always)]
        pub const fn swj_cfg(&self) -> super::vals::SwjCfg {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::SwjCfg::from_bits(val as u8)
        }
        #[doc = "Serial wire JTAG configuration (must be set to NoOp to leave it unchanged!)"]
        #[inline(always)]
        pub fn set_swj_cfg(&mut self, val: super::vals::SwjCfg) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "SPI3/I2S3 remapping"]
        #[inline(always)]
        pub const fn spi3_remap(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3/I2S3 remapping"]
        #[inline(always)]
        pub fn set_spi3_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "TIM2 internal trigger 1 remapping"]
        #[inline(always)]
        pub const fn tim2itr1_iremap(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 internal trigger 1 remapping"]
        #[inline(always)]
        pub fn set_tim2itr1_iremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[inline(always)]
        pub const fn ptp_pps_remap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[inline(always)]
        pub fn set_ptp_pps_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Mapr {
        #[inline(always)]
        fn default() -> Mapr {
            Mapr(0)
        }
    }
    impl core::fmt::Debug for Mapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mapr")
                .field("spi1_remap", &self.spi1_remap())
                .field("i2c1_remap", &self.i2c1_remap())
                .field("usart1_remap", &self.usart1_remap())
                .field("usart2_remap", &self.usart2_remap())
                .field("usart3_remap", &self.usart3_remap())
                .field("tim1_remap", &self.tim1_remap())
                .field("tim2_remap", &self.tim2_remap())
                .field("tim3_remap", &self.tim3_remap())
                .field("tim4_remap", &self.tim4_remap())
                .field("can1_remap", &self.can1_remap())
                .field("pd01_remap", &self.pd01_remap())
                .field("tim5ch4_iremap", &self.tim5ch4_iremap())
                .field("adc1_etrginj_remap", &self.adc1_etrginj_remap())
                .field("adc1_etrgreg_remap", &self.adc1_etrgreg_remap())
                .field("adc2_etrginj_remap", &self.adc2_etrginj_remap())
                .field("adc2_etrgreg_remap", &self.adc2_etrgreg_remap())
                .field("eth_remap", &self.eth_remap())
                .field("can2_remap", &self.can2_remap())
                .field("mii_rmii_sel", &self.mii_rmii_sel())
                .field("swj_cfg", &self.swj_cfg())
                .field("spi3_remap", &self.spi3_remap())
                .field("tim2itr1_iremap", &self.tim2itr1_iremap())
                .field("ptp_pps_remap", &self.ptp_pps_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mapr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mapr {{ spi1_remap: {=bool:?}, i2c1_remap: {=bool:?}, usart1_remap: {=bool:?}, usart2_remap: {=bool:?}, usart3_remap: {=u8:?}, tim1_remap: {=u8:?}, tim2_remap: {=u8:?}, tim3_remap: {=u8:?}, tim4_remap: {=bool:?}, can1_remap: {=u8:?}, pd01_remap: {=bool:?}, tim5ch4_iremap: {=bool:?}, adc1_etrginj_remap: {=bool:?}, adc1_etrgreg_remap: {=bool:?}, adc2_etrginj_remap: {=bool:?}, adc2_etrgreg_remap: {=bool:?}, eth_remap: {=bool:?}, can2_remap: {=bool:?}, mii_rmii_sel: {=bool:?}, swj_cfg: {:?}, spi3_remap: {=bool:?}, tim2itr1_iremap: {=bool:?}, ptp_pps_remap: {=bool:?} }}" , self . spi1_remap () , self . i2c1_remap () , self . usart1_remap () , self . usart2_remap () , self . usart3_remap () , self . tim1_remap () , self . tim2_remap () , self . tim3_remap () , self . tim4_remap () , self . can1_remap () , self . pd01_remap () , self . tim5ch4_iremap () , self . adc1_etrginj_remap () , self . adc1_etrgreg_remap () , self . adc2_etrginj_remap () , self . adc2_etrgreg_remap () , self . eth_remap () , self . can2_remap () , self . mii_rmii_sel () , self . swj_cfg () , self . spi3_remap () , self . tim2itr1_iremap () , self . ptp_pps_remap ())
        }
    }
    #[doc = "AF remap and debug I/O configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mapr2(pub u32);
    impl Mapr2 {
        #[doc = "TIM15 remapping"]
        #[inline(always)]
        pub const fn tim15_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 remapping"]
        #[inline(always)]
        pub fn set_tim15_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM16 remapping"]
        #[inline(always)]
        pub const fn tim16_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 remapping"]
        #[inline(always)]
        pub fn set_tim16_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM17 remapping"]
        #[inline(always)]
        pub const fn tim17_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 remapping"]
        #[inline(always)]
        pub fn set_tim17_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CEC remapping"]
        #[inline(always)]
        pub const fn cec_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CEC remapping"]
        #[inline(always)]
        pub fn set_cec_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM1 DMA remapping"]
        #[inline(always)]
        pub const fn tim1_dma_remap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 DMA remapping"]
        #[inline(always)]
        pub fn set_tim1_dma_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM9 remapping"]
        #[inline(always)]
        pub const fn tim9_remap(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 remapping"]
        #[inline(always)]
        pub fn set_tim9_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM10 remapping"]
        #[inline(always)]
        pub const fn tim10_remap(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 remapping"]
        #[inline(always)]
        pub fn set_tim10_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM11 remapping"]
        #[inline(always)]
        pub const fn tim11_remap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 remapping"]
        #[inline(always)]
        pub fn set_tim11_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM13 remapping"]
        #[inline(always)]
        pub const fn tim13_remap(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 remapping"]
        #[inline(always)]
        pub fn set_tim13_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM14 remapping"]
        #[inline(always)]
        pub const fn tim14_remap(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 remapping"]
        #[inline(always)]
        pub fn set_tim14_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "NADV connect/disconnect"]
        #[inline(always)]
        pub const fn fsmc_nadv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "NADV connect/disconnect"]
        #[inline(always)]
        pub fn set_fsmc_nadv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM67_DAC DMA remapping"]
        #[inline(always)]
        pub const fn tim67_dac_dma_remap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM67_DAC DMA remapping"]
        #[inline(always)]
        pub fn set_tim67_dac_dma_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM12 remapping"]
        #[inline(always)]
        pub const fn tim12_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 remapping"]
        #[inline(always)]
        pub fn set_tim12_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Miscellaneous features remapping"]
        #[inline(always)]
        pub const fn misc_remap(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Miscellaneous features remapping"]
        #[inline(always)]
        pub fn set_misc_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Mapr2 {
        #[inline(always)]
        fn default() -> Mapr2 {
            Mapr2(0)
        }
    }
    impl core::fmt::Debug for Mapr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mapr2")
                .field("tim15_remap", &self.tim15_remap())
                .field("tim16_remap", &self.tim16_remap())
                .field("tim17_remap", &self.tim17_remap())
                .field("cec_remap", &self.cec_remap())
                .field("tim1_dma_remap", &self.tim1_dma_remap())
                .field("tim9_remap", &self.tim9_remap())
                .field("tim10_remap", &self.tim10_remap())
                .field("tim11_remap", &self.tim11_remap())
                .field("tim13_remap", &self.tim13_remap())
                .field("tim14_remap", &self.tim14_remap())
                .field("fsmc_nadv", &self.fsmc_nadv())
                .field("tim67_dac_dma_remap", &self.tim67_dac_dma_remap())
                .field("tim12_remap", &self.tim12_remap())
                .field("misc_remap", &self.misc_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mapr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mapr2 {{ tim15_remap: {=bool:?}, tim16_remap: {=bool:?}, tim17_remap: {=bool:?}, cec_remap: {=bool:?}, tim1_dma_remap: {=bool:?}, tim9_remap: {=bool:?}, tim10_remap: {=bool:?}, tim11_remap: {=bool:?}, tim13_remap: {=bool:?}, tim14_remap: {=bool:?}, fsmc_nadv: {=bool:?}, tim67_dac_dma_remap: {=bool:?}, tim12_remap: {=bool:?}, misc_remap: {=bool:?} }}" , self . tim15_remap () , self . tim16_remap () , self . tim17_remap () , self . cec_remap () , self . tim1_dma_remap () , self . tim9_remap () , self . tim10_remap () , self . tim11_remap () , self . tim13_remap () , self . tim14_remap () , self . fsmc_nadv () , self . tim67_dac_dma_remap () , self . tim12_remap () , self . misc_remap ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SwjCfg {
        #[doc = "Full SWJ (JTAG-DP + SW-DP) (Reset state)"]
        RESET = 0x0,
        #[doc = "Full SWJ (JTAG-DP + SW-DP) but without NJTRST"]
        NO_JNT_RST = 0x01,
        #[doc = "JTAG-DP Disabled and SW-DP Enabled"]
        JTAG_DISABLE = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "JTAG-DP Disabled and SW-DP Disabled"]
        DISABLE = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "Sets all bits to 1, indicating that the configuration should remain unchanged"]
        NO_OP = 0x07,
    }
    impl SwjCfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SwjCfg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SwjCfg {
        #[inline(always)]
        fn from(val: u8) -> SwjCfg {
            SwjCfg::from_bits(val)
        }
    }
    impl From<SwjCfg> for u8 {
        #[inline(always)]
        fn from(val: SwjCfg) -> u8 {
            SwjCfg::to_bits(val)
        }
    }
}
