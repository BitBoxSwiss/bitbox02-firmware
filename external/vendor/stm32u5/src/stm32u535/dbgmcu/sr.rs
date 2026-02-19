///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `AP_PRESENT` reader - AP_PRESENT
pub type AP_PRESENT_R = crate::FieldReader;
///Field `AP_LOCKED` reader - AP_LOCKED
pub type AP_LOCKED_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - AP_PRESENT
    #[inline(always)]
    pub fn ap_present(&self) -> AP_PRESENT_R {
        AP_PRESENT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - AP_LOCKED
    #[inline(always)]
    pub fn ap_locked(&self) -> AP_LOCKED_R {
        AP_LOCKED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ap_present", &self.ap_present())
            .field("ap_locked", &self.ap_locked())
            .finish()
    }
}
/**DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DBGMCU:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
