///Register `DBGMCU_DBG_AUTH_HOST` reader
pub type R = crate::R<DBGMCU_DBG_AUTH_HOSTrs>;
///Field `AUTH_KEY` reader - AUTH_KEY
pub type AUTH_KEY_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - AUTH_KEY
    #[inline(always)]
    pub fn auth_key(&self) -> AUTH_KEY_R {
        AUTH_KEY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_DBG_AUTH_HOST")
            .field("auth_key", &self.auth_key())
            .finish()
    }
}
/**DBGMCU debug host authentication register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DBGMCU:DBGMCU_DBG_AUTH_HOST)*/
pub struct DBGMCU_DBG_AUTH_HOSTrs;
impl crate::RegisterSpec for DBGMCU_DBG_AUTH_HOSTrs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_dbg_auth_host::R`](R) reader structure
impl crate::Readable for DBGMCU_DBG_AUTH_HOSTrs {}
///`reset()` method sets DBGMCU_DBG_AUTH_HOST to value 0
impl crate::Resettable for DBGMCU_DBG_AUTH_HOSTrs {}
