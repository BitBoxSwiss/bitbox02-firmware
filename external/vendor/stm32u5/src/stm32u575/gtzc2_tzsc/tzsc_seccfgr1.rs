///Register `TZSC_SECCFGR1` reader
pub type R = crate::R<TZSC_SECCFGR1rs>;
///Register `TZSC_SECCFGR1` writer
pub type W = crate::W<TZSC_SECCFGR1rs>;
///Field `SPI3SEC` reader - secure access mode for SPI3
pub type SPI3SEC_R = crate::BitReader;
///Field `SPI3SEC` writer - secure access mode for SPI3
pub type SPI3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1SEC` reader - secure access mode for LPUART1
pub type LPUART1SEC_R = crate::BitReader;
///Field `LPUART1SEC` writer - secure access mode for LPUART1
pub type LPUART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3SEC` reader - secure access mode for I2C3
pub type I2C3SEC_R = crate::BitReader;
///Field `I2C3SEC` writer - secure access mode for I2C3
pub type I2C3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1SEC` reader - secure access mode for LPTIM1
pub type LPTIM1SEC_R = crate::BitReader;
///Field `LPTIM1SEC` writer - secure access mode for LPTIM1
pub type LPTIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3SEC` reader - secure access mode for LPTIM3
pub type LPTIM3SEC_R = crate::BitReader;
///Field `LPTIM3SEC` writer - secure access mode for LPTIM3
pub type LPTIM3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4SEC` reader - secure access mode for LPTIM4
pub type LPTIM4SEC_R = crate::BitReader;
///Field `LPTIM4SEC` writer - secure access mode for LPTIM4
pub type LPTIM4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPSEC` reader - secure access mode for OPAMP
pub type OPAMPSEC_R = crate::BitReader;
///Field `OPAMPSEC` writer - secure access mode for OPAMP
pub type OPAMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPSEC` reader - secure access mode for COMP
pub type COMPSEC_R = crate::BitReader;
///Field `COMPSEC` writer - secure access mode for COMP
pub type COMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4SEC` reader - secure access mode for ADC4
pub type ADC4SEC_R = crate::BitReader;
///Field `ADC4SEC` writer - secure access mode for ADC4
pub type ADC4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFSEC` reader - secure access mode for VREFBUF
pub type VREFBUFSEC_R = crate::BitReader;
///Field `VREFBUFSEC` writer - secure access mode for VREFBUF
pub type VREFBUFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1SEC` reader - secure access mode for DAC1
pub type DAC1SEC_R = crate::BitReader;
///Field `DAC1SEC` writer - secure access mode for DAC1
pub type DAC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1SEC` reader - secure access mode for ADF1
pub type ADF1SEC_R = crate::BitReader;
///Field `ADF1SEC` writer - secure access mode for ADF1
pub type ADF1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for SPI3
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure access mode for I2C3
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure access mode for LPTIM3
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure access mode for LPTIM4
    #[inline(always)]
    pub fn lptim4sec(&self) -> LPTIM4SEC_R {
        LPTIM4SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for OPAMP
    #[inline(always)]
    pub fn opampsec(&self) -> OPAMPSEC_R {
        OPAMPSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure access mode for COMP
    #[inline(always)]
    pub fn compsec(&self) -> COMPSEC_R {
        COMPSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - secure access mode for ADC4
    #[inline(always)]
    pub fn adc4sec(&self) -> ADC4SEC_R {
        ADC4SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - secure access mode for DAC1
    #[inline(always)]
    pub fn dac1sec(&self) -> DAC1SEC_R {
        DAC1SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure access mode for ADF1
    #[inline(always)]
    pub fn adf1sec(&self) -> ADF1SEC_R {
        ADF1SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_SECCFGR1")
            .field("spi3sec", &self.spi3sec())
            .field("lpuart1sec", &self.lpuart1sec())
            .field("i2c3sec", &self.i2c3sec())
            .field("lptim1sec", &self.lptim1sec())
            .field("lptim3sec", &self.lptim3sec())
            .field("lptim4sec", &self.lptim4sec())
            .field("opampsec", &self.opampsec())
            .field("compsec", &self.compsec())
            .field("adc4sec", &self.adc4sec())
            .field("vrefbufsec", &self.vrefbufsec())
            .field("dac1sec", &self.dac1sec())
            .field("adf1sec", &self.adf1sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for SPI3
    #[inline(always)]
    pub fn spi3sec(&mut self) -> SPI3SEC_W<TZSC_SECCFGR1rs> {
        SPI3SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<TZSC_SECCFGR1rs> {
        LPUART1SEC_W::new(self, 1)
    }
    ///Bit 2 - secure access mode for I2C3
    #[inline(always)]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<TZSC_SECCFGR1rs> {
        I2C3SEC_W::new(self, 2)
    }
    ///Bit 3 - secure access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<TZSC_SECCFGR1rs> {
        LPTIM1SEC_W::new(self, 3)
    }
    ///Bit 4 - secure access mode for LPTIM3
    #[inline(always)]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W<TZSC_SECCFGR1rs> {
        LPTIM3SEC_W::new(self, 4)
    }
    ///Bit 5 - secure access mode for LPTIM4
    #[inline(always)]
    pub fn lptim4sec(&mut self) -> LPTIM4SEC_W<TZSC_SECCFGR1rs> {
        LPTIM4SEC_W::new(self, 5)
    }
    ///Bit 6 - secure access mode for OPAMP
    #[inline(always)]
    pub fn opampsec(&mut self) -> OPAMPSEC_W<TZSC_SECCFGR1rs> {
        OPAMPSEC_W::new(self, 6)
    }
    ///Bit 7 - secure access mode for COMP
    #[inline(always)]
    pub fn compsec(&mut self) -> COMPSEC_W<TZSC_SECCFGR1rs> {
        COMPSEC_W::new(self, 7)
    }
    ///Bit 8 - secure access mode for ADC4
    #[inline(always)]
    pub fn adc4sec(&mut self) -> ADC4SEC_W<TZSC_SECCFGR1rs> {
        ADC4SEC_W::new(self, 8)
    }
    ///Bit 9 - secure access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<TZSC_SECCFGR1rs> {
        VREFBUFSEC_W::new(self, 9)
    }
    ///Bit 11 - secure access mode for DAC1
    #[inline(always)]
    pub fn dac1sec(&mut self) -> DAC1SEC_W<TZSC_SECCFGR1rs> {
        DAC1SEC_W::new(self, 11)
    }
    ///Bit 12 - secure access mode for ADF1
    #[inline(always)]
    pub fn adf1sec(&mut self) -> ADF1SEC_W<TZSC_SECCFGR1rs> {
        ADF1SEC_W::new(self, 12)
    }
}
/**TZSC secure configuration register 1

You can [`read`](crate::Reg::read) this register and get [`tzsc_seccfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_seccfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZSC:TZSC_SECCFGR1)*/
pub struct TZSC_SECCFGR1rs;
impl crate::RegisterSpec for TZSC_SECCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_seccfgr1::R`](R) reader structure
impl crate::Readable for TZSC_SECCFGR1rs {}
///`write(|w| ..)` method takes [`tzsc_seccfgr1::W`](W) writer structure
impl crate::Writable for TZSC_SECCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_SECCFGR1 to value 0
impl crate::Resettable for TZSC_SECCFGR1rs {}
