///Register `M3ICR` reader
pub type R = crate::R<M3ICRrs>;
///Register `M3ICR` writer
pub type W = crate::W<M3ICRrs>;
///Field `CSEDC` reader - CSEDC
pub type CSEDC_R = crate::BitReader;
///Field `CSEDC` writer - CSEDC
pub type CSEDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDED` reader - CDED
pub type CDED_R = crate::BitReader;
///Field `CDED` writer - CDED
pub type CDED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CSEDC
    #[inline(always)]
    pub fn csedc(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CDED
    #[inline(always)]
    pub fn cded(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3ICR")
            .field("csedc", &self.csedc())
            .field("cded", &self.cded())
            .finish()
    }
}
impl W {
    ///Bit 0 - CSEDC
    #[inline(always)]
    pub fn csedc(&mut self) -> CSEDC_W<M3ICRrs> {
        CSEDC_W::new(self, 0)
    }
    ///Bit 1 - CDED
    #[inline(always)]
    pub fn cded(&mut self) -> CDED_W<M3ICRrs> {
        CDED_W::new(self, 1)
    }
}
/**RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`m3icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RAMCFG:M3ICR)*/
pub struct M3ICRrs;
impl crate::RegisterSpec for M3ICRrs {
    type Ux = u32;
}
///`read()` method returns [`m3icr::R`](R) reader structure
impl crate::Readable for M3ICRrs {}
///`write(|w| ..)` method takes [`m3icr::W`](W) writer structure
impl crate::Writable for M3ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3ICR to value 0
impl crate::Resettable for M3ICRrs {}
