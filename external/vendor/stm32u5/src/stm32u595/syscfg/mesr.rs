///Register `MESR` reader
pub type R = crate::R<MESRrs>;
///Register `MESR` writer
pub type W = crate::W<MESRrs>;
///Field `MCLR` reader - MCLR
pub type MCLR_R = crate::BitReader;
///Field `MCLR` writer - MCLR
pub type MCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPMEE` reader - IPMEE
pub type IPMEE_R = crate::BitReader;
///Field `IPMEE` writer - IPMEE
pub type IPMEE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MCLR
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - IPMEE
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESR")
            .field("ipmee", &self.ipmee())
            .field("mclr", &self.mclr())
            .finish()
    }
}
impl W {
    ///Bit 0 - MCLR
    #[inline(always)]
    pub fn mclr(&mut self) -> MCLR_W<MESRrs> {
        MCLR_W::new(self, 0)
    }
    ///Bit 16 - IPMEE
    #[inline(always)]
    pub fn ipmee(&mut self) -> IPMEE_W<MESRrs> {
        IPMEE_W::new(self, 16)
    }
}
/**memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#SYSCFG:MESR)*/
pub struct MESRrs;
impl crate::RegisterSpec for MESRrs {
    type Ux = u32;
}
///`read()` method returns [`mesr::R`](R) reader structure
impl crate::Readable for MESRrs {}
///`write(|w| ..)` method takes [`mesr::W`](W) writer structure
impl crate::Writable for MESRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MESR to value 0
impl crate::Resettable for MESRrs {}
