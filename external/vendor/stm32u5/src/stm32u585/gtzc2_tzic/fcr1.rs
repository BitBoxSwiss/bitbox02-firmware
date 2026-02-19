///Register `FCR1` writer
pub type W = crate::W<FCR1rs>;
///Field `CSPI3F` writer - clear the illegal access flag for SPI3
pub type CSPI3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPUART1F` writer - clear the illegal access flag for LPUART1
pub type CLPUART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C3F` writer - clear the illegal access flag for I2C3
pub type CI2C3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1
pub type CLPTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM3F` writer - clear the illegal access flag for LPTIM3
pub type CLPTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM4F` writer - clear the illegal access flag for LPTIM4
pub type CLPTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COPAMPF` writer - clear the illegal access flag for OPAMP
pub type COPAMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCOMPF` writer - clear the illegal access flag for COMP
pub type CCOMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CADC4F` writer - clear the illegal access flag for ADC4
pub type CADC4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CVREFBUFF` writer - clear the illegal access flag for VREFBUF
pub type CVREFBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDAC1F` writer - clear the illegal access flag for DAC1
pub type CDAC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CADF1F` writer - clear the illegal access flag for ADF1
pub type CADF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for SPI3
    #[inline(always)]
    pub fn cspi3f(&mut self) -> CSPI3F_W<FCR1rs> {
        CSPI3F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for LPUART1
    #[inline(always)]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<FCR1rs> {
        CLPUART1F_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for I2C3
    #[inline(always)]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<FCR1rs> {
        CI2C3F_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for LPTIM1
    #[inline(always)]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<FCR1rs> {
        CLPTIM1F_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for LPTIM3
    #[inline(always)]
    pub fn clptim3f(&mut self) -> CLPTIM3F_W<FCR1rs> {
        CLPTIM3F_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for LPTIM4
    #[inline(always)]
    pub fn clptim4f(&mut self) -> CLPTIM4F_W<FCR1rs> {
        CLPTIM4F_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for OPAMP
    #[inline(always)]
    pub fn copampf(&mut self) -> COPAMPF_W<FCR1rs> {
        COPAMPF_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for COMP
    #[inline(always)]
    pub fn ccompf(&mut self) -> CCOMPF_W<FCR1rs> {
        CCOMPF_W::new(self, 7)
    }
    ///Bit 8 - clear the illegal access flag for ADC4
    #[inline(always)]
    pub fn cadc4f(&mut self) -> CADC4F_W<FCR1rs> {
        CADC4F_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for VREFBUF
    #[inline(always)]
    pub fn cvrefbuff(&mut self) -> CVREFBUFF_W<FCR1rs> {
        CVREFBUFF_W::new(self, 9)
    }
    ///Bit 11 - clear the illegal access flag for DAC1
    #[inline(always)]
    pub fn cdac1f(&mut self) -> CDAC1F_W<FCR1rs> {
        CDAC1F_W::new(self, 11)
    }
    ///Bit 12 - clear the illegal access flag for ADF1
    #[inline(always)]
    pub fn cadf1f(&mut self) -> CADF1F_W<FCR1rs> {
        CADF1F_W::new(self, 12)
    }
}
/**TZIC flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC2_TZIC:FCR1)*/
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr1::W`](W) writer structure
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1rs {}
