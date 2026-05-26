///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `SPI3F` reader - illegal access flag for SPI3
pub type SPI3F_R = crate::BitReader;
///Field `LPUART1F` reader - illegal access flag for LPUART1
pub type LPUART1F_R = crate::BitReader;
///Field `I2C3F` reader - illegal access flag for I2C3
pub type I2C3F_R = crate::BitReader;
///Field `LPTIM1F` reader - illegal access flag for LPTIM1
pub type LPTIM1F_R = crate::BitReader;
///Field `LPTIM3F` reader - illegal access flag for LPTIM3
pub type LPTIM3F_R = crate::BitReader;
///Field `LPTIM4F` reader - illegal access flag for LPTIM4
pub type LPTIM4F_R = crate::BitReader;
///Field `OPAMPF` reader - illegal access flag for OPAMP
pub type OPAMPF_R = crate::BitReader;
///Field `COMPF` reader - illegal access flag for COMP
pub type COMPF_R = crate::BitReader;
///Field `ADC4F` reader - illegal access flag for ADC4
pub type ADC4F_R = crate::BitReader;
///Field `VREFBUFF` reader - illegal access flag for VREFBUF
pub type VREFBUFF_R = crate::BitReader;
///Field `DAC1F` reader - illegal access flag for DAC1
pub type DAC1F_R = crate::BitReader;
///Field `ADF1F` reader - illegal access flag for ADF1
pub type ADF1F_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for SPI3
    #[inline(always)]
    pub fn spi3f(&self) -> SPI3F_R {
        SPI3F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for LPUART1
    #[inline(always)]
    pub fn lpuart1f(&self) -> LPUART1F_R {
        LPUART1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for I2C3
    #[inline(always)]
    pub fn i2c3f(&self) -> I2C3F_R {
        I2C3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for LPTIM1
    #[inline(always)]
    pub fn lptim1f(&self) -> LPTIM1F_R {
        LPTIM1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for LPTIM3
    #[inline(always)]
    pub fn lptim3f(&self) -> LPTIM3F_R {
        LPTIM3F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for LPTIM4
    #[inline(always)]
    pub fn lptim4f(&self) -> LPTIM4F_R {
        LPTIM4F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for OPAMP
    #[inline(always)]
    pub fn opampf(&self) -> OPAMPF_R {
        OPAMPF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for COMP
    #[inline(always)]
    pub fn compf(&self) -> COMPF_R {
        COMPF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for ADC4
    #[inline(always)]
    pub fn adc4f(&self) -> ADC4F_R {
        ADC4F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for VREFBUF
    #[inline(always)]
    pub fn vrefbuff(&self) -> VREFBUFF_R {
        VREFBUFF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for DAC1
    #[inline(always)]
    pub fn dac1f(&self) -> DAC1F_R {
        DAC1F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for ADF1
    #[inline(always)]
    pub fn adf1f(&self) -> ADF1F_R {
        ADF1F_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("spi3f", &self.spi3f())
            .field("lpuart1f", &self.lpuart1f())
            .field("i2c3f", &self.i2c3f())
            .field("lptim1f", &self.lptim1f())
            .field("lptim3f", &self.lptim3f())
            .field("lptim4f", &self.lptim4f())
            .field("opampf", &self.opampf())
            .field("compf", &self.compf())
            .field("adc4f", &self.adc4f())
            .field("vrefbuff", &self.vrefbuff())
            .field("dac1f", &self.dac1f())
            .field("adf1f", &self.adf1f())
            .finish()
    }
}
/**TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
