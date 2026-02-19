///Register `ATSEEDR` reader
pub type R = crate::R<ATSEEDRrs>;
///Register `ATSEEDR` writer
pub type W = crate::W<ATSEEDRrs>;
///Field `SEED` reader - SEED
pub type SEED_R = crate::FieldReader<u32>;
///Field `SEED` writer - SEED
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SEED
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATSEEDR")
            .field("seed", &self.seed())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SEED
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W<ATSEEDRrs> {
        SEED_W::new(self, 0)
    }
}
/**TAMP active tamper seed register

You can [`read`](crate::Reg::read) this register and get [`atseedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TAMP:ATSEEDR)*/
pub struct ATSEEDRrs;
impl crate::RegisterSpec for ATSEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`atseedr::R`](R) reader structure
impl crate::Readable for ATSEEDRrs {}
///`write(|w| ..)` method takes [`atseedr::W`](W) writer structure
impl crate::Writable for ATSEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ATSEEDR to value 0
impl crate::Resettable for ATSEEDRrs {}
