///Register `CREL` reader
pub type R = crate::R<CRELrs>;
///Field `DAY` reader - Timestamp Day
pub type DAY_R = crate::FieldReader;
///Field `MON` reader - Timestamp Month
pub type MON_R = crate::FieldReader;
///Field `YEAR` reader - Timestamp Year
pub type YEAR_R = crate::FieldReader;
///Field `SUBSTEP` reader - Sub-step of Core release
pub type SUBSTEP_R = crate::FieldReader;
///Field `STEP` reader - Step of Core release
pub type STEP_R = crate::FieldReader;
///Field `REL` reader - Core release
pub type REL_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Timestamp Day
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Timestamp Month
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Timestamp Year
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Sub-step of Core release
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Step of Core release
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Core release
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CREL")
            .field("rel", &self.rel())
            .field("step", &self.step())
            .field("substep", &self.substep())
            .field("year", &self.year())
            .field("mon", &self.mon())
            .field("day", &self.day())
            .finish()
    }
}
/**FDCAN Core Release Register

You can [`read`](crate::Reg::read) this register and get [`crel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FDCAN1_RAM:CREL)*/
pub struct CRELrs;
impl crate::RegisterSpec for CRELrs {
    type Ux = u32;
}
///`read()` method returns [`crel::R`](R) reader structure
impl crate::Readable for CRELrs {}
///`reset()` method sets CREL to value 0x3214_1218
impl crate::Resettable for CRELrs {
    const RESET_VALUE: u32 = 0x3214_1218;
}
