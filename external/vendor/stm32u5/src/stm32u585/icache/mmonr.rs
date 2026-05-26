///Register `MMONR` reader
pub type R = crate::R<MMONRrs>;
///Field `MISSMON` reader - MISSMON
pub type MISSMON_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - MISSMON
    #[inline(always)]
    pub fn missmon(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMONR")
            .field("missmon", &self.missmon())
            .finish()
    }
}
/**ICACHE miss monitor register

You can [`read`](crate::Reg::read) this register and get [`mmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ICACHE:MMONR)*/
pub struct MMONRrs;
impl crate::RegisterSpec for MMONRrs {
    type Ux = u32;
}
///`read()` method returns [`mmonr::R`](R) reader structure
impl crate::Readable for MMONRrs {}
///`reset()` method sets MMONR to value 0
impl crate::Resettable for MMONRrs {}
