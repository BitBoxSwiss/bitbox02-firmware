///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DEN` reader - Operational amplifier Enable
pub type DEN_R = crate::BitReader;
///Field `DEN` writer - Operational amplifier Enable
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEN` reader - OPALPM
pub type SEN_R = crate::BitReader;
///Field `SEN` writer - OPALPM
pub type SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OPALPM
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("den", &self.den())
            .field("sen", &self.sen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W<CRrs> {
        DEN_W::new(self, 0)
    }
    ///Bit 1 - OPALPM
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W<CRrs> {
        SEN_W::new(self, 1)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DLYBOS1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
