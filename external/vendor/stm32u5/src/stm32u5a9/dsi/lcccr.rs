///Register `LCCCR` reader
pub type R = crate::R<LCCCRrs>;
///Field `COLC` reader - Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit
pub type COLC_R = crate::FieldReader;
///Field `LPE` reader - Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations.
pub type LPE_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations.
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCCCR")
            .field("colc", &self.colc())
            .field("lpe", &self.lpe())
            .finish()
    }
}
/**DSI Host LTDC current color coding register

You can [`read`](crate::Reg::read) this register and get [`lcccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:LCCCR)*/
pub struct LCCCRrs;
impl crate::RegisterSpec for LCCCRrs {
    type Ux = u32;
}
///`read()` method returns [`lcccr::R`](R) reader structure
impl crate::Readable for LCCCRrs {}
///`reset()` method sets LCCCR to value 0
impl crate::Resettable for LCCCRrs {}
