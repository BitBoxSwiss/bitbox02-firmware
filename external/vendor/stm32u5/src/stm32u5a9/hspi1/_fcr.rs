///Register `_FCR` writer
pub type W = crate::W<_FCRrs>;
///Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the HSPI_SR register.
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the HSPI_SR register.
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the HSPI_SR register.
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the HSPI_SR register.
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<_FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the HSPI_SR register.
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W<_FCRrs> {
        CTEF_W::new(self, 0)
    }
    ///Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the HSPI_SR register.
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W<_FCRrs> {
        CTCF_W::new(self, 1)
    }
    ///Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the HSPI_SR register.
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W<_FCRrs> {
        CSMF_W::new(self, 3)
    }
    ///Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the HSPI_SR register.
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W<_FCRrs> {
        CTOF_W::new(self, 4)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_FCR)*/
pub struct _FCRrs;
impl crate::RegisterSpec for _FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`_fcr::W`](W) writer structure
impl crate::Writable for _FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _FCR to value 0
impl crate::Resettable for _FCRrs {}
