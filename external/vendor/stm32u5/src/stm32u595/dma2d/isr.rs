///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `TEIF` reader - Transfer error interrupt flag
pub type TEIF_R = crate::BitReader;
///Field `TCIF` reader - Transfer complete interrupt flag
pub type TCIF_R = crate::BitReader;
///Field `TWIF` reader - Transfer watermark interrupt flag
pub type TWIF_R = crate::BitReader;
///Field `CAEIF` reader - CLUT access error interrupt flag
pub type CAEIF_R = crate::BitReader;
///Field `CTCIF` reader - CLUT transfer complete interrupt flag
pub type CTCIF_R = crate::BitReader;
///Field `CEIF` reader - Configuration error interrupt flag
pub type CEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Transfer error interrupt flag
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete interrupt flag
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer watermark interrupt flag
    #[inline(always)]
    pub fn twif(&self) -> TWIF_R {
        TWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CLUT access error interrupt flag
    #[inline(always)]
    pub fn caeif(&self) -> CAEIF_R {
        CAEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CLUT transfer complete interrupt flag
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configuration error interrupt flag
    #[inline(always)]
    pub fn ceif(&self) -> CEIF_R {
        CEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("ceif", &self.ceif())
            .field("ctcif", &self.ctcif())
            .field("caeif", &self.caeif())
            .field("twif", &self.twif())
            .field("tcif", &self.tcif())
            .field("teif", &self.teif())
            .finish()
    }
}
/**Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DMA2D:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
