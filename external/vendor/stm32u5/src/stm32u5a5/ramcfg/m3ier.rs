///Register `M3IER` reader
pub type R = crate::R<M3IERrs>;
///Register `M3IER` writer
pub type W = crate::W<M3IERrs>;
///Field `SEIE` reader - SEIE
pub type SEIE_R = crate::BitReader;
///Field `SEIE` writer - SEIE
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEIE` reader - DEIE
pub type DEIE_R = crate::BitReader;
///Field `DEIE` writer - DEIE
pub type DEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCNMI` reader - ECCNMI
pub type ECCNMI_R = crate::BitReader;
///Field `ECCNMI` writer - ECCNMI
pub type ECCNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SEIE
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DEIE
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - ECCNMI
    #[inline(always)]
    pub fn eccnmi(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3IER")
            .field("seie", &self.seie())
            .field("deie", &self.deie())
            .field("eccnmi", &self.eccnmi())
            .finish()
    }
}
impl W {
    ///Bit 0 - SEIE
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W<M3IERrs> {
        SEIE_W::new(self, 0)
    }
    ///Bit 1 - DEIE
    #[inline(always)]
    pub fn deie(&mut self) -> DEIE_W<M3IERrs> {
        DEIE_W::new(self, 1)
    }
    ///Bit 3 - ECCNMI
    #[inline(always)]
    pub fn eccnmi(&mut self) -> ECCNMI_W<M3IERrs> {
        ECCNMI_W::new(self, 3)
    }
}
/**RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m3ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RAMCFG:M3IER)*/
pub struct M3IERrs;
impl crate::RegisterSpec for M3IERrs {
    type Ux = u32;
}
///`read()` method returns [`m3ier::R`](R) reader structure
impl crate::Readable for M3IERrs {}
///`write(|w| ..)` method takes [`m3ier::W`](W) writer structure
impl crate::Writable for M3IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3IER to value 0
impl crate::Resettable for M3IERrs {}
