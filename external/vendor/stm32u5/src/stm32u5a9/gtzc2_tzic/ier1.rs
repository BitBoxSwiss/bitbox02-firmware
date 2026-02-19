///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
///Field `SPI3IE` reader - illegal access interrupt enable for SPI3
pub type SPI3IE_R = crate::BitReader;
///Field `SPI3IE` writer - illegal access interrupt enable for SPI3
pub type SPI3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1IE` reader - illegal access interrupt enable for LPUART1
pub type LPUART1IE_R = crate::BitReader;
///Field `LPUART1IE` writer - illegal access interrupt enable for LPUART1
pub type LPUART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3IE` reader - illegal access interrupt enable for I2C3
pub type I2C3IE_R = crate::BitReader;
///Field `I2C3IE` writer - illegal access interrupt enable for I2C3
pub type I2C3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1IE` reader - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_R = crate::BitReader;
///Field `LPTIM1IE` writer - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3IE` reader - illegal access interrupt enable for LPTIM3
pub type LPTIM3IE_R = crate::BitReader;
///Field `LPTIM3IE` writer - illegal access interrupt enable for LPTIM3
pub type LPTIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4IE` reader - illegal access interrupt enable for LPTIM4
pub type LPTIM4IE_R = crate::BitReader;
///Field `LPTIM4IE` writer - illegal access interrupt enable for LPTIM4
pub type LPTIM4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPIE` reader - illegal access interrupt enable for OPAMP
pub type OPAMPIE_R = crate::BitReader;
///Field `OPAMPIE` writer - illegal access interrupt enable for OPAMP
pub type OPAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPIE` reader - illegal access interrupt enable for COMP
pub type COMPIE_R = crate::BitReader;
///Field `COMPIE` writer - illegal access interrupt enable for COMP
pub type COMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC2IE` reader - illegal access interrupt enable for ADC2
pub type ADC2IE_R = crate::BitReader;
///Field `ADC2IE` writer - illegal access interrupt enable for ADC2
pub type ADC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFIE` reader - illegal access interrupt enable for VREFBUF
pub type VREFBUFIE_R = crate::BitReader;
///Field `VREFBUFIE` writer - illegal access interrupt enable for VREFBUF
pub type VREFBUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1IE` reader - illegal access interrupt enable for DAC1
pub type DAC1IE_R = crate::BitReader;
///Field `DAC1IE` writer - illegal access interrupt enable for DAC1
pub type DAC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1IE` reader - illegal access interrupt enable for ADF1
pub type ADF1IE_R = crate::BitReader;
///Field `ADF1IE` writer - illegal access interrupt enable for ADF1
pub type ADF1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for SPI3
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for LPUART1
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for LPTIM3
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for LPTIM4
    #[inline(always)]
    pub fn lptim4ie(&self) -> LPTIM4IE_R {
        LPTIM4IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for OPAMP
    #[inline(always)]
    pub fn opampie(&self) -> OPAMPIE_R {
        OPAMPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for COMP
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for ADC2
    #[inline(always)]
    pub fn adc2ie(&self) -> ADC2IE_R {
        ADC2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for VREFBUF
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for DAC1
    #[inline(always)]
    pub fn dac1ie(&self) -> DAC1IE_R {
        DAC1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for ADF1
    #[inline(always)]
    pub fn adf1ie(&self) -> ADF1IE_R {
        ADF1IE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("spi3ie", &self.spi3ie())
            .field("lpuart1ie", &self.lpuart1ie())
            .field("i2c3ie", &self.i2c3ie())
            .field("lptim1ie", &self.lptim1ie())
            .field("lptim3ie", &self.lptim3ie())
            .field("lptim4ie", &self.lptim4ie())
            .field("opampie", &self.opampie())
            .field("compie", &self.compie())
            .field("adc2ie", &self.adc2ie())
            .field("vrefbufie", &self.vrefbufie())
            .field("dac1ie", &self.dac1ie())
            .field("adf1ie", &self.adf1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for SPI3
    #[inline(always)]
    pub fn spi3ie(&mut self) -> SPI3IE_W<IER1rs> {
        SPI3IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for LPUART1
    #[inline(always)]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<IER1rs> {
        LPUART1IE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<IER1rs> {
        I2C3IE_W::new(self, 2)
    }
    ///Bit 3 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<IER1rs> {
        LPTIM1IE_W::new(self, 3)
    }
    ///Bit 4 - illegal access interrupt enable for LPTIM3
    #[inline(always)]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W<IER1rs> {
        LPTIM3IE_W::new(self, 4)
    }
    ///Bit 5 - illegal access interrupt enable for LPTIM4
    #[inline(always)]
    pub fn lptim4ie(&mut self) -> LPTIM4IE_W<IER1rs> {
        LPTIM4IE_W::new(self, 5)
    }
    ///Bit 6 - illegal access interrupt enable for OPAMP
    #[inline(always)]
    pub fn opampie(&mut self) -> OPAMPIE_W<IER1rs> {
        OPAMPIE_W::new(self, 6)
    }
    ///Bit 7 - illegal access interrupt enable for COMP
    #[inline(always)]
    pub fn compie(&mut self) -> COMPIE_W<IER1rs> {
        COMPIE_W::new(self, 7)
    }
    ///Bit 8 - illegal access interrupt enable for ADC2
    #[inline(always)]
    pub fn adc2ie(&mut self) -> ADC2IE_W<IER1rs> {
        ADC2IE_W::new(self, 8)
    }
    ///Bit 9 - illegal access interrupt enable for VREFBUF
    #[inline(always)]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<IER1rs> {
        VREFBUFIE_W::new(self, 9)
    }
    ///Bit 11 - illegal access interrupt enable for DAC1
    #[inline(always)]
    pub fn dac1ie(&mut self) -> DAC1IE_W<IER1rs> {
        DAC1IE_W::new(self, 11)
    }
    ///Bit 12 - illegal access interrupt enable for ADF1
    #[inline(always)]
    pub fn adf1ie(&mut self) -> ADF1IE_W<IER1rs> {
        ADF1IE_W::new(self, 12)
    }
}
/**TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC2_TZIC:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1rs {}
