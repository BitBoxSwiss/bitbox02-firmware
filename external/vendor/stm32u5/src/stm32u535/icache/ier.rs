///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `BSYENDIE` reader - BSYENDIE
pub type BSYENDIE_R = crate::BitReader;
///Field `BSYENDIE` writer - BSYENDIE
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("bsyendie", &self.bsyendie())
            .field("errie", &self.errie())
            .finish()
    }
}
impl W {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<IERrs> {
        BSYENDIE_W::new(self, 1)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<IERrs> {
        ERRIE_W::new(self, 2)
    }
}
/**ICACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ICACHE:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
