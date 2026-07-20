///Register `HMONR` reader
pub type R = crate::R<HMONRrs>;
///Field `HITMON` reader - HITMON
pub type HITMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HITMON
    #[inline(always)]
    pub fn hitmon(&self) -> HITMON_R {
        HITMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMONR")
            .field("hitmon", &self.hitmon())
            .finish()
    }
}
/**ICACHE hit monitor register

You can [`read`](crate::Reg::read) this register and get [`hmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ICACHE:HMONR)*/
pub struct HMONRrs;
impl crate::RegisterSpec for HMONRrs {
    type Ux = u32;
}
///`read()` method returns [`hmonr::R`](R) reader structure
impl crate::Readable for HMONRrs {}
///`reset()` method sets HMONR to value 0
impl crate::Resettable for HMONRrs {}
