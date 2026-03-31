///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `TIM1F` reader - illegal access flag for TIM1
pub type TIM1F_R = crate::BitReader;
///Field `SPI1F` reader - illegal access flag for SPI1
pub type SPI1F_R = crate::BitReader;
///Field `TIM8F` reader - illegal access flag for TIM8
pub type TIM8F_R = crate::BitReader;
///Field `USART1F` reader - illegal access flag for USART1
pub type USART1F_R = crate::BitReader;
///Field `TIM15F` reader - illegal access flag for TIM5
pub type TIM15F_R = crate::BitReader;
///Field `TIM16F` reader - illegal access flag for TIM6
pub type TIM16F_R = crate::BitReader;
///Field `TIM17F` reader - illegal access flag for TIM7
pub type TIM17F_R = crate::BitReader;
///Field `SAI1F` reader - illegal access flag for SAI1
pub type SAI1F_R = crate::BitReader;
///Field `SAI2F` reader - illegal access flag for SAI2
pub type SAI2F_R = crate::BitReader;
///Field `LTDCF` reader - illegal access flag for LTDC
pub type LTDCF_R = crate::BitReader;
///Field `DSIF` reader - illegal access flag for DSI
pub type DSIF_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for TIM1
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for SPI1
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for TIM8
    #[inline(always)]
    pub fn tim8f(&self) -> TIM8F_R {
        TIM8F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for USART1
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TIM5
    #[inline(always)]
    pub fn tim15f(&self) -> TIM15F_R {
        TIM15F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for TIM6
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for TIM7
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for SAI1
    #[inline(always)]
    pub fn sai1f(&self) -> SAI1F_R {
        SAI1F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for SAI2
    #[inline(always)]
    pub fn sai2f(&self) -> SAI2F_R {
        SAI2F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for LTDC
    #[inline(always)]
    pub fn ltdcf(&self) -> LTDCF_R {
        LTDCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for DSI
    #[inline(always)]
    pub fn dsif(&self) -> DSIF_R {
        DSIF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("tim1f", &self.tim1f())
            .field("spi1f", &self.spi1f())
            .field("tim8f", &self.tim8f())
            .field("usart1f", &self.usart1f())
            .field("tim15f", &self.tim15f())
            .field("tim16f", &self.tim16f())
            .field("tim17f", &self.tim17f())
            .field("sai1f", &self.sai1f())
            .field("sai2f", &self.sai2f())
            .field("ltdcf", &self.ltdcf())
            .field("dsif", &self.dsif())
            .finish()
    }
}
/**TZIC status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
