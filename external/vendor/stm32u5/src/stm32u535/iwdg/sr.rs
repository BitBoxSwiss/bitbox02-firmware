///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `PVU` reader - Watchdog prescaler value update
pub type PVU_R = crate::BitReader;
///Field `RVU` reader - Watchdog counter reload value update
pub type RVU_R = crate::BitReader;
///Field `WVU` reader - Watchdog counter window value update
pub type WVU_R = crate::BitReader;
///Field `EWU` reader - Watchdog interrupt comparator value update
pub type EWU_R = crate::BitReader;
///Field `EWIF` reader - Watchdog Early interrupt flag
pub type EWIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Watchdog prescaler value update
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Watchdog counter window value update
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Watchdog interrupt comparator value update
    #[inline(always)]
    pub fn ewu(&self) -> EWU_R {
        EWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 14 - Watchdog Early interrupt flag
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ewif", &self.ewif())
            .field("ewu", &self.ewu())
            .field("wvu", &self.wvu())
            .field("rvu", &self.rvu())
            .field("pvu", &self.pvu())
            .finish()
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#IWDG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
