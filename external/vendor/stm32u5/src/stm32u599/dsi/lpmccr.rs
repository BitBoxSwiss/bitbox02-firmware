///Register `LPMCCR` reader
pub type R = crate::R<LPMCCRrs>;
///Field `VLPSIZE` reader - VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode.
pub type VLPSIZE_R = crate::FieldReader;
///Field `LPSIZE` reader - Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode.
pub type LPSIZE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode.
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode.
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMCCR")
            .field("vlpsize", &self.vlpsize())
            .field("lpsize", &self.lpsize())
            .finish()
    }
}
/**DSI Host low-power mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`lpmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:LPMCCR)*/
pub struct LPMCCRrs;
impl crate::RegisterSpec for LPMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`lpmccr::R`](R) reader structure
impl crate::Readable for LPMCCRrs {}
///`reset()` method sets LPMCCR to value 0
impl crate::Resettable for LPMCCRrs {}
