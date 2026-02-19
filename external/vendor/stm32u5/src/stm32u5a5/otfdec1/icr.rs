///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Field `SEIF` reader - SEIF
pub type SEIF_R = crate::BitReader;
///Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag clear
pub type XONEIF_R = crate::BitReader;
///Field `KEIF` reader - KEIF
pub type KEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - SEIF
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Execute-only execute-Never Error Interrupt Flag clear
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - KEIF
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("seif", &self.seif())
            .field("xoneif", &self.xoneif())
            .field("keif", &self.keif())
            .finish()
    }
}
/**OTFDEC interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTFDEC1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
