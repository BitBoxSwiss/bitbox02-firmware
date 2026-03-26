///Register `SECHDPCR` reader
pub type R = crate::R<SECHDPCRrs>;
///Register `SECHDPCR` writer
pub type W = crate::W<SECHDPCRrs>;
///Field `HDP1_ACCDIS` reader - HDP1 area access disable When set, this bit is only cleared by a system reset.
pub type HDP1_ACCDIS_R = crate::BitReader;
///Field `HDP1_ACCDIS` writer - HDP1 area access disable When set, this bit is only cleared by a system reset.
pub type HDP1_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDP2_ACCDIS` reader - HDP2 area access disable When set, this bit is only cleared by a system reset.
pub type HDP2_ACCDIS_R = crate::BitReader;
///Field `HDP2_ACCDIS` writer - HDP2 area access disable When set, this bit is only cleared by a system reset.
pub type HDP2_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECHDPCR")
            .field("hdp1_accdis", &self.hdp1_accdis())
            .field("hdp2_accdis", &self.hdp2_accdis())
            .finish()
    }
}
impl W {
    ///Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W<SECHDPCRrs> {
        HDP1_ACCDIS_W::new(self, 0)
    }
    ///Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W<SECHDPCRrs> {
        HDP2_ACCDIS_W::new(self, 1)
    }
}
/**FLASH secure HDP control register

You can [`read`](crate::Reg::read) this register and get [`sechdpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sechdpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FLASH:SECHDPCR)*/
pub struct SECHDPCRrs;
impl crate::RegisterSpec for SECHDPCRrs {
    type Ux = u32;
}
///`read()` method returns [`sechdpcr::R`](R) reader structure
impl crate::Readable for SECHDPCRrs {}
///`write(|w| ..)` method takes [`sechdpcr::W`](W) writer structure
impl crate::Writable for SECHDPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECHDPCR to value 0
impl crate::Resettable for SECHDPCRrs {}
