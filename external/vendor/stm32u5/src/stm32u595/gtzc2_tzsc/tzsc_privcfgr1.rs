///Register `TZSC_PRIVCFGR1` reader
pub type R = crate::R<TZSC_PRIVCFGR1rs>;
///Register `TZSC_PRIVCFGR1` writer
pub type W = crate::W<TZSC_PRIVCFGR1rs>;
///Field `SPI3PRIV` reader - privileged access mode for SPI3
pub type SPI3PRIV_R = crate::BitReader;
///Field `SPI3PRIV` writer - privileged access mode for SPI3
pub type SPI3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1PRIV` reader - privileged access mode for LPUART1
pub type LPUART1PRIV_R = crate::BitReader;
///Field `LPUART1PRIV` writer - privileged access mode for LPUART1
pub type LPUART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3PRIV` reader - privileged access mode for I2C3
pub type I2C3PRIV_R = crate::BitReader;
///Field `I2C3PRIV` writer - privileged access mode for I2C3
pub type I2C3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1
pub type LPTIM1PRIV_R = crate::BitReader;
///Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1
pub type LPTIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3PRIV` reader - privileged access mode for LPTIM3
pub type LPTIM3PRIV_R = crate::BitReader;
///Field `LPTIM3PRIV` writer - privileged access mode for LPTIM3
pub type LPTIM3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4PRIV` reader - privileged access mode for LPTIM4
pub type LPTIM4PRIV_R = crate::BitReader;
///Field `LPTIM4PRIV` writer - privileged access mode for LPTIM4
pub type LPTIM4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPPRIV` reader - privileged access mode for OPAMP
pub type OPAMPPRIV_R = crate::BitReader;
///Field `OPAMPPRIV` writer - privileged access mode for OPAMP
pub type OPAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPPRIV` reader - privileged access mode for COMP
pub type COMPPRIV_R = crate::BitReader;
///Field `COMPPRIV` writer - privileged access mode for COMP
pub type COMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC2PRIV` reader - privileged access mode for ADC2
pub type ADC2PRIV_R = crate::BitReader;
///Field `ADC2PRIV` writer - privileged access mode for ADC2
pub type ADC2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFPRIV` reader - privileged access mode for VREFBUF
pub type VREFBUFPRIV_R = crate::BitReader;
///Field `VREFBUFPRIV` writer - privileged access mode for VREFBUF
pub type VREFBUFPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1PRIV` reader - privileged access mode for DAC1
pub type DAC1PRIV_R = crate::BitReader;
///Field `DAC1PRIV` writer - privileged access mode for DAC1
pub type DAC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1PRIV` reader - privileged access mode for ADF1
pub type ADF1PRIV_R = crate::BitReader;
///Field `ADF1PRIV` writer - privileged access mode for ADF1
pub type ADF1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged access mode for I2C3
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for LPTIM3
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for LPTIM4
    #[inline(always)]
    pub fn lptim4priv(&self) -> LPTIM4PRIV_R {
        LPTIM4PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for OPAMP
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged access mode for COMP
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for ADC2
    #[inline(always)]
    pub fn adc2priv(&self) -> ADC2PRIV_R {
        ADC2PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for DAC1
    #[inline(always)]
    pub fn dac1priv(&self) -> DAC1PRIV_R {
        DAC1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - privileged access mode for ADF1
    #[inline(always)]
    pub fn adf1priv(&self) -> ADF1PRIV_R {
        ADF1PRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_PRIVCFGR1")
            .field("spi3priv", &self.spi3priv())
            .field("lpuart1priv", &self.lpuart1priv())
            .field("i2c3priv", &self.i2c3priv())
            .field("lptim1priv", &self.lptim1priv())
            .field("lptim3priv", &self.lptim3priv())
            .field("lptim4priv", &self.lptim4priv())
            .field("opamppriv", &self.opamppriv())
            .field("comppriv", &self.comppriv())
            .field("adc2priv", &self.adc2priv())
            .field("vrefbufpriv", &self.vrefbufpriv())
            .field("dac1priv", &self.dac1priv())
            .field("adf1priv", &self.adf1priv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<TZSC_PRIVCFGR1rs> {
        SPI3PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<TZSC_PRIVCFGR1rs> {
        LPUART1PRIV_W::new(self, 1)
    }
    ///Bit 2 - privileged access mode for I2C3
    #[inline(always)]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W<TZSC_PRIVCFGR1rs> {
        I2C3PRIV_W::new(self, 2)
    }
    ///Bit 3 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM1PRIV_W::new(self, 3)
    }
    ///Bit 4 - privileged access mode for LPTIM3
    #[inline(always)]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM3PRIV_W::new(self, 4)
    }
    ///Bit 5 - privileged access mode for LPTIM4
    #[inline(always)]
    pub fn lptim4priv(&mut self) -> LPTIM4PRIV_W<TZSC_PRIVCFGR1rs> {
        LPTIM4PRIV_W::new(self, 5)
    }
    ///Bit 6 - privileged access mode for OPAMP
    #[inline(always)]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<TZSC_PRIVCFGR1rs> {
        OPAMPPRIV_W::new(self, 6)
    }
    ///Bit 7 - privileged access mode for COMP
    #[inline(always)]
    pub fn comppriv(&mut self) -> COMPPRIV_W<TZSC_PRIVCFGR1rs> {
        COMPPRIV_W::new(self, 7)
    }
    ///Bit 8 - privileged access mode for ADC2
    #[inline(always)]
    pub fn adc2priv(&mut self) -> ADC2PRIV_W<TZSC_PRIVCFGR1rs> {
        ADC2PRIV_W::new(self, 8)
    }
    ///Bit 9 - privileged access mode for VREFBUF
    #[inline(always)]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<TZSC_PRIVCFGR1rs> {
        VREFBUFPRIV_W::new(self, 9)
    }
    ///Bit 11 - privileged access mode for DAC1
    #[inline(always)]
    pub fn dac1priv(&mut self) -> DAC1PRIV_W<TZSC_PRIVCFGR1rs> {
        DAC1PRIV_W::new(self, 11)
    }
    ///Bit 12 - privileged access mode for ADF1
    #[inline(always)]
    pub fn adf1priv(&mut self) -> ADF1PRIV_W<TZSC_PRIVCFGR1rs> {
        ADF1PRIV_W::new(self, 12)
    }
}
/**TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GTZC2_TZSC:TZSC_PRIVCFGR1)*/
pub struct TZSC_PRIVCFGR1rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_privcfgr1::R`](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR1rs {}
///`write(|w| ..)` method takes [`tzsc_privcfgr1::W`](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_PRIVCFGR1 to value 0
impl crate::Resettable for TZSC_PRIVCFGR1rs {}
