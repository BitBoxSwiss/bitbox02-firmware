///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `CFG1` reader - peripheral hardware configuration 1
pub type CFG1_R = crate::FieldReader;
///Field `CFG2` reader - peripheral hardware configuration 2
pub type CFG2_R = crate::FieldReader;
///Field `CFG3` reader - peripheral hardware configuration 3
pub type CFG3_R = crate::FieldReader;
///Field `CFG4` reader - peripheral hardware configuration 4
pub type CFG4_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - peripheral hardware configuration 1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - peripheral hardware configuration 2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - peripheral hardware configuration 3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:31 - peripheral hardware configuration 4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .finish()
    }
}
/**LPTIM peripheral hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPTIM1:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0
impl crate::Resettable for HWCFGR1rs {}
