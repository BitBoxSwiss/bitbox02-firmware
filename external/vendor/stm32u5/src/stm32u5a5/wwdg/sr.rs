///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Early wakeup interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFR {
    ///0: The EWI Interrupt Service Routine has been serviced
    Finished = 0,
    ///1: The EWI Interrupt Service Routine has been triggered
    Pending = 1,
}
impl From<EWIFR> for bool {
    #[inline(always)]
    fn from(variant: EWIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIF` reader - Early wakeup interrupt flag
pub type EWIF_R = crate::BitReader<EWIFR>;
impl EWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWIFR {
        match self.bits {
            false => EWIFR::Finished,
            true => EWIFR::Pending,
        }
    }
    ///The EWI Interrupt Service Routine has been serviced
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == EWIFR::Finished
    }
    ///The EWI Interrupt Service Routine has been triggered
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIFR::Pending
    }
}
/**Early wakeup interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFW {
    ///0: The EWI Interrupt Service Routine has been serviced
    Finished = 0,
}
impl From<EWIFW> for bool {
    #[inline(always)]
    fn from(variant: EWIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIF` writer - Early wakeup interrupt flag
pub type EWIF_W<'a, REG> = crate::BitWriter0C<'a, REG, EWIFW>;
impl<'a, REG> EWIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The EWI Interrupt Service Routine has been serviced
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(EWIFW::Finished)
    }
}
impl R {
    ///Bit 0 - Early wakeup interrupt flag
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR").field("ewif", &self.ewif()).finish()
    }
}
impl W {
    ///Bit 0 - Early wakeup interrupt flag
    #[inline(always)]
    pub fn ewif(&mut self) -> EWIF_W<SRrs> {
        EWIF_W::new(self, 0)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#WWDG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x01;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
