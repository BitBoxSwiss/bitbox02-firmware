///Register `RAM3IER` reader
pub type R = crate::R<RAM3IERrs>;
///Register `RAM3IER` writer
pub type W = crate::W<RAM3IERrs>;
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
        f.debug_struct("RAM3IER")
            .field("seie", &self.seie())
            .field("deie", &self.deie())
            .field("eccnmi", &self.eccnmi())
            .finish()
    }
}
impl W {
    ///Bit 0 - SEIE
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W<RAM3IERrs> {
        SEIE_W::new(self, 0)
    }
    ///Bit 1 - DEIE
    #[inline(always)]
    pub fn deie(&mut self) -> DEIE_W<RAM3IERrs> {
        DEIE_W::new(self, 1)
    }
    ///Bit 3 - ECCNMI
    #[inline(always)]
    pub fn eccnmi(&mut self) -> ECCNMI_W<RAM3IERrs> {
        ECCNMI_W::new(self, 3)
    }
}
/**RAMCFG SRAM x interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ram3ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram3ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RAMCFG:RAM3IER)*/
pub struct RAM3IERrs;
impl crate::RegisterSpec for RAM3IERrs {
    type Ux = u32;
}
///`read()` method returns [`ram3ier::R`](R) reader structure
impl crate::Readable for RAM3IERrs {}
///`write(|w| ..)` method takes [`ram3ier::W`](W) writer structure
impl crate::Writable for RAM3IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM3IER to value 0
impl crate::Resettable for RAM3IERrs {}
