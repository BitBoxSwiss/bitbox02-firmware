///Register `RAM2ICR` reader
pub type R = crate::R<RAM2ICRrs>;
///Register `RAM2ICR` writer
pub type W = crate::W<RAM2ICRrs>;
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
        f.debug_struct("RAM2ICR")
            .field("csedc", &self.csedc())
            .field("cded", &self.cded())
            .finish()
    }
}
impl W {
    ///Bit 0 - CSEDC
    #[inline(always)]
    pub fn csedc(&mut self) -> CSEDC_W<RAM2ICRrs> {
        CSEDC_W::new(self, 0)
    }
    ///Bit 1 - CDED
    #[inline(always)]
    pub fn cded(&mut self) -> CDED_W<RAM2ICRrs> {
        CDED_W::new(self, 1)
    }
}
/**RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`ram2icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM2ICR)*/
pub struct RAM2ICRrs;
impl crate::RegisterSpec for RAM2ICRrs {
    type Ux = u32;
}
///`read()` method returns [`ram2icr::R`](R) reader structure
impl crate::Readable for RAM2ICRrs {}
///`write(|w| ..)` method takes [`ram2icr::W`](W) writer structure
impl crate::Writable for RAM2ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM2ICR to value 0
impl crate::Resettable for RAM2ICRrs {}
