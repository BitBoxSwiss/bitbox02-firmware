///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TAMP1F` reader - TAMP1F
pub type TAMP1F_R = crate::BitReader;
///Field `TAMP2F` reader - TAMP2F
pub type TAMP2F_R = crate::BitReader;
///Field `TAMP3F` reader - TAMP3F
pub type TAMP3F_R = crate::BitReader;
///Field `TAMP4F` reader - TAMP4F
pub type TAMP4F_R = crate::BitReader;
///Field `TAMP5F` reader - TAMP5F
pub type TAMP5F_R = crate::BitReader;
///Field `TAMP6F` reader - TAMP6F
pub type TAMP6F_R = crate::BitReader;
///Field `TAMP7F` reader - TAMP7F
pub type TAMP7F_R = crate::BitReader;
///Field `TAMP8F` reader - TAMP8F
pub type TAMP8F_R = crate::BitReader;
///Field `CITAMP1F` reader - CITAMP1F
pub type CITAMP1F_R = crate::BitReader;
///Field `CITAMP2F` reader - CITAMP2F
pub type CITAMP2F_R = crate::BitReader;
///Field `ITAMP3F` reader - ITAMP3F
pub type ITAMP3F_R = crate::BitReader;
///Field `ITAMP5F` reader - ITAMP5F
pub type ITAMP5F_R = crate::BitReader;
///Field `ITAMP6F` reader - ITAMP6F
pub type ITAMP6F_R = crate::BitReader;
///Field `ITAMP7F` reader - ITAMP7F
pub type ITAMP7F_R = crate::BitReader;
///Field `ITAMP8F` reader - ITAMP8F
pub type ITAMP8F_R = crate::BitReader;
///Field `ITAMP9F` reader - ITAMP9F
pub type ITAMP9F_R = crate::BitReader;
///Field `CITAMP11F` reader - CITAMP11F
pub type CITAMP11F_R = crate::BitReader;
///Field `ITAMP12F` reader - ITAMP12F
pub type ITAMP12F_R = crate::BitReader;
///Field `ITAMP13IE` reader - ITAMP13IE
pub type ITAMP13IE_R = crate::BitReader;
impl R {
    ///Bit 0 - TAMP1F
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2F
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3F
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TAMP4F
    #[inline(always)]
    pub fn tamp4f(&self) -> TAMP4F_R {
        TAMP4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TAMP5F
    #[inline(always)]
    pub fn tamp5f(&self) -> TAMP5F_R {
        TAMP5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TAMP6F
    #[inline(always)]
    pub fn tamp6f(&self) -> TAMP6F_R {
        TAMP6F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMP7F
    #[inline(always)]
    pub fn tamp7f(&self) -> TAMP7F_R {
        TAMP7F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TAMP8F
    #[inline(always)]
    pub fn tamp8f(&self) -> TAMP8F_R {
        TAMP8F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - CITAMP1F
    #[inline(always)]
    pub fn citamp1f(&self) -> CITAMP1F_R {
        CITAMP1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CITAMP2F
    #[inline(always)]
    pub fn citamp2f(&self) -> CITAMP2F_R {
        CITAMP2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ITAMP3F
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5F
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6F
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ITAMP7F
    #[inline(always)]
    pub fn itamp7f(&self) -> ITAMP7F_R {
        ITAMP7F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ITAMP8F
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ITAMP9F
    #[inline(always)]
    pub fn itamp9f(&self) -> ITAMP9F_R {
        ITAMP9F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - CITAMP11F
    #[inline(always)]
    pub fn citamp11f(&self) -> CITAMP11F_R {
        CITAMP11F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ITAMP12F
    #[inline(always)]
    pub fn itamp12f(&self) -> ITAMP12F_R {
        ITAMP12F_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ITAMP13IE
    #[inline(always)]
    pub fn itamp13ie(&self) -> ITAMP13IE_R {
        ITAMP13IE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tamp3f", &self.tamp3f())
            .field("tamp4f", &self.tamp4f())
            .field("tamp5f", &self.tamp5f())
            .field("tamp6f", &self.tamp6f())
            .field("tamp7f", &self.tamp7f())
            .field("tamp8f", &self.tamp8f())
            .field("citamp1f", &self.citamp1f())
            .field("citamp2f", &self.citamp2f())
            .field("itamp3f", &self.itamp3f())
            .field("itamp5f", &self.itamp5f())
            .field("itamp6f", &self.itamp6f())
            .field("itamp7f", &self.itamp7f())
            .field("itamp8f", &self.itamp8f())
            .field("itamp9f", &self.itamp9f())
            .field("citamp11f", &self.citamp11f())
            .field("itamp12f", &self.itamp12f())
            .field("itamp13ie", &self.itamp13ie())
            .finish()
    }
}
/**TAMP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#TAMP:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
