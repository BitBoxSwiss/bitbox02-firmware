///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `CCF` reader - Computation complete flag
pub type CCF_R = crate::BitReader;
///Field `RDERR` reader - Read error flag
pub type RDERR_R = crate::BitReader;
///Field `WRERR` reader - Write error flag
pub type WRERR_R = crate::BitReader;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
///Field `KEYVALID` reader - Key Valid flag
pub type KEYVALID_R = crate::BitReader;
impl R {
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read error flag
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write error flag
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Key Valid flag
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("keyvalid", &self.keyvalid())
            .field("busy", &self.busy())
            .field("wrerr", &self.wrerr())
            .field("rderr", &self.rderr())
            .field("ccf", &self.ccf())
            .finish()
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SAES:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
