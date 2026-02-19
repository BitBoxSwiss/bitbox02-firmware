///Register `CSLOCKR` reader
pub type R = crate::R<CSLOCKRrs>;
///Register `CSLOCKR` writer
pub type W = crate::W<CSLOCKRrs>;
///Field `LOCKSVTAIRCR` reader - LOCKSVTAIRCR
pub type LOCKSVTAIRCR_R = crate::BitReader;
///Field `LOCKSVTAIRCR` writer - LOCKSVTAIRCR
pub type LOCKSVTAIRCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKSMPU` reader - LOCKSMPU
pub type LOCKSMPU_R = crate::BitReader;
///Field `LOCKSMPU` writer - LOCKSMPU
pub type LOCKSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKSAU` reader - LOCKSAU
pub type LOCKSAU_R = crate::BitReader;
///Field `LOCKSAU` writer - LOCKSAU
pub type LOCKSAU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LOCKSVTAIRCR
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LOCKSMPU
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LOCKSAU
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSLOCKR")
            .field("locksvtaircr", &self.locksvtaircr())
            .field("locksmpu", &self.locksmpu())
            .field("locksau", &self.locksau())
            .finish()
    }
}
impl W {
    ///Bit 0 - LOCKSVTAIRCR
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<CSLOCKRrs> {
        LOCKSVTAIRCR_W::new(self, 0)
    }
    ///Bit 1 - LOCKSMPU
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<CSLOCKRrs> {
        LOCKSMPU_W::new(self, 1)
    }
    ///Bit 2 - LOCKSAU
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W<CSLOCKRrs> {
        LOCKSAU_W::new(self, 2)
    }
}
/**SYSCFG CPU secure lock register

You can [`read`](crate::Reg::read) this register and get [`cslockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cslockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SYSCFG:CSLOCKR)*/
pub struct CSLOCKRrs;
impl crate::RegisterSpec for CSLOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`cslockr::R`](R) reader structure
impl crate::Readable for CSLOCKRrs {}
///`write(|w| ..)` method takes [`cslockr::W`](W) writer structure
impl crate::Writable for CSLOCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSLOCKR to value 0
impl crate::Resettable for CSLOCKRrs {}
