///Register `WDATA` writer
pub type W = crate::W<WDATArs>;
///Field `ARG` writer - Function input arguments
pub type ARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<WDATArs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Function input arguments
    #[inline(always)]
    pub fn arg(&mut self) -> ARG_W<WDATArs> {
        ARG_W::new(self, 0)
    }
}
/**FMAC Write Data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#CORDIC:WDATA)*/
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wdata::W`](W) writer structure
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATArs {}
