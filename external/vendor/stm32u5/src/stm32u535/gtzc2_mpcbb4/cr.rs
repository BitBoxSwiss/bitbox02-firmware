///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `GLOCK` reader - lock the control register of the MPCBB until next reset
pub type GLOCK_R = crate::BitReader;
///Field `GLOCK` writer - lock the control register of the MPCBB until next reset
pub type GLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INVSECSTATE` reader - SRAMx clocks security state
pub type INVSECSTATE_R = crate::BitReader;
///Field `INVSECSTATE` writer - SRAMx clocks security state
pub type INVSECSTATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRWILADIS` reader - secure read/write illegal access disable
pub type SRWILADIS_R = crate::BitReader;
///Field `SRWILADIS` writer - secure read/write illegal access disable
pub type SRWILADIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - lock the control register of the MPCBB until next reset
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 30 - SRAMx clocks security state
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - secure read/write illegal access disable
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("glock", &self.glock())
            .field("invsecstate", &self.invsecstate())
            .field("srwiladis", &self.srwiladis())
            .finish()
    }
}
impl W {
    ///Bit 0 - lock the control register of the MPCBB until next reset
    #[inline(always)]
    pub fn glock(&mut self) -> GLOCK_W<CRrs> {
        GLOCK_W::new(self, 0)
    }
    ///Bit 30 - SRAMx clocks security state
    #[inline(always)]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W<CRrs> {
        INVSECSTATE_W::new(self, 30)
    }
    ///Bit 31 - secure read/write illegal access disable
    #[inline(always)]
    pub fn srwiladis(&mut self) -> SRWILADIS_W<CRrs> {
        SRWILADIS_W::new(self, 31)
    }
}
/**MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GTZC2_MPCBB4:CR)*/
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
