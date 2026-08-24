///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `CCF` reader - Computation complete flag
pub type CCF_R = crate::BitReader;
///Field `RWEIF` reader - Read or write error interrupt flag
pub type RWEIF_R = crate::BitReader;
///Field `KEIF` reader - Key error interrupt flag
pub type KEIF_R = crate::BitReader;
///Field `RNGEIF` reader - RNGEIF
pub type RNGEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt flag
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt flag
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGEIF
    #[inline(always)]
    pub fn rngeif(&self) -> RNGEIF_R {
        RNGEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rngeif", &self.rngeif())
            .field("keif", &self.keif())
            .field("rweif", &self.rweif())
            .field("ccf", &self.ccf())
            .finish()
    }
}
/**interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#SAES:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
