///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `SEIF` reader - Security Error Interrupt Flag status
pub type SEIF_R = crate::BitReader;
///Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag status
pub type XONEIF_R = crate::BitReader;
///Field `KEIF` reader - Key Error Interrupt Flag status
pub type KEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Security Error Interrupt Flag status
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Execute-only execute-Never Error Interrupt Flag status
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key Error Interrupt Flag status
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("seif", &self.seif())
            .field("xoneif", &self.xoneif())
            .field("keif", &self.keif())
            .finish()
    }
}
/**OTFDEC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTFDEC1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
