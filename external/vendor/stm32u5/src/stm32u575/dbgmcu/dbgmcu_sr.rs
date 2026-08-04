///Register `DBGMCU_SR` reader
pub type R = crate::R<DBGMCU_SRrs>;
///Field `AP_PRESENT` reader - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present
pub type AP_PRESENT_R = crate::FieldReader;
///Field `AP_LOCKED` reader - DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled
pub type AP_LOCKED_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present
    #[inline(always)]
    pub fn ap_present(&self) -> AP_PRESENT_R {
        AP_PRESENT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled
    #[inline(always)]
    pub fn ap_locked(&self) -> AP_LOCKED_R {
        AP_LOCKED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_SR")
            .field("ap_present", &self.ap_present())
            .field("ap_locked", &self.ap_locked())
            .finish()
    }
}
/**DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DBGMCU:DBGMCU_SR)*/
pub struct DBGMCU_SRrs;
impl crate::RegisterSpec for DBGMCU_SRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_sr::R`](R) reader structure
impl crate::Readable for DBGMCU_SRrs {}
///`reset()` method sets DBGMCU_SR to value 0x01
impl crate::Resettable for DBGMCU_SRrs {
    const RESET_VALUE: u32 = 0x01;
}
