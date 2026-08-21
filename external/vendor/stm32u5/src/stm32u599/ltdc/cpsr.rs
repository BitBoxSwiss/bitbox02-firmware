///Register `CPSR` reader
pub type R = crate::R<CPSRrs>;
///Field `CYPOS` reader - current Y position These bits return the current Y position.
pub type CYPOS_R = crate::FieldReader<u16>;
///Field `CXPOS` reader - current X position These bits return the current X position.
pub type CXPOS_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - current Y position These bits return the current Y position.
    #[inline(always)]
    pub fn cypos(&self) -> CYPOS_R {
        CYPOS_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - current X position These bits return the current X position.
    #[inline(always)]
    pub fn cxpos(&self) -> CXPOS_R {
        CXPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPSR")
            .field("cypos", &self.cypos())
            .field("cxpos", &self.cxpos())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:CPSR)*/
pub struct CPSRrs;
impl crate::RegisterSpec for CPSRrs {
    type Ux = u32;
}
///`read()` method returns [`cpsr::R`](R) reader structure
impl crate::Readable for CPSRrs {}
///`reset()` method sets CPSR to value 0
impl crate::Resettable for CPSRrs {}
