///Register `RSSCMDR` reader
pub type R = crate::R<RSSCMDRrs>;
///Register `RSSCMDR` writer
pub type W = crate::W<RSSCMDRrs>;
///Field `RSSCMD` reader - RSS commands
pub type RSSCMD_R = crate::FieldReader<u16>;
///Field `RSSCMD` writer - RSS commands
pub type RSSCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - RSS commands
    #[inline(always)]
    pub fn rsscmd(&self) -> RSSCMD_R {
        RSSCMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSSCMDR")
            .field("rsscmd", &self.rsscmd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - RSS commands
    #[inline(always)]
    pub fn rsscmd(&mut self) -> RSSCMD_W<RSSCMDRrs> {
        RSSCMD_W::new(self, 0)
    }
}
/**RSS command register

You can [`read`](crate::Reg::read) this register and get [`rsscmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsscmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SYSCFG:RSSCMDR)*/
pub struct RSSCMDRrs;
impl crate::RegisterSpec for RSSCMDRrs {
    type Ux = u32;
}
///`read()` method returns [`rsscmdr::R`](R) reader structure
impl crate::Readable for RSSCMDRrs {}
///`write(|w| ..)` method takes [`rsscmdr::W`](W) writer structure
impl crate::Writable for RSSCMDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSSCMDR to value 0
impl crate::Resettable for RSSCMDRrs {}
