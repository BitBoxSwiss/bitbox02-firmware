///Register `CCVR` reader
pub type R = crate::R<CCVRrs>;
///Field `NCV1` reader - NCV1
pub type NCV1_R = crate::FieldReader;
///Field `PCV1` reader - PCV1
pub type PCV1_R = crate::FieldReader;
///Field `NCV2` reader - NCV2
pub type NCV2_R = crate::FieldReader;
///Field `PCV2` reader - PCV2
pub type PCV2_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - NCV1
    #[inline(always)]
    pub fn ncv1(&self) -> NCV1_R {
        NCV1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PCV1
    #[inline(always)]
    pub fn pcv1(&self) -> PCV1_R {
        PCV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NCV2
    #[inline(always)]
    pub fn ncv2(&self) -> NCV2_R {
        NCV2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PCV2
    #[inline(always)]
    pub fn pcv2(&self) -> PCV2_R {
        PCV2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCVR")
            .field("ncv1", &self.ncv1())
            .field("pcv1", &self.pcv1())
            .field("ncv2", &self.ncv2())
            .field("pcv2", &self.pcv2())
            .finish()
    }
}
/**compensation cell value register

You can [`read`](crate::Reg::read) this register and get [`ccvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SYSCFG:CCVR)*/
pub struct CCVRrs;
impl crate::RegisterSpec for CCVRrs {
    type Ux = u32;
}
///`read()` method returns [`ccvr::R`](R) reader structure
impl crate::Readable for CCVRrs {}
///`reset()` method sets CCVR to value 0
impl crate::Resettable for CCVRrs {}
