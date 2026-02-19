///Register `HCFG` reader
pub type R = crate::R<HCFGrs>;
///Register `HCFG` writer
pub type W = crate::W<HCFGrs>;
///Field `FSLSPCS` reader - FSLSPCS
pub type FSLSPCS_R = crate::FieldReader;
///Field `FSLSPCS` writer - FSLSPCS
pub type FSLSPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FSLSS` reader - FSLSS
pub type FSLSS_R = crate::BitReader;
impl R {
    ///Bits 0:1 - FSLSPCS
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - FSLSS
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFG")
            .field("fslspcs", &self.fslspcs())
            .field("fslss", &self.fslss())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FSLSPCS
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<HCFGrs> {
        FSLSPCS_W::new(self, 0)
    }
}
/**This register configures the core after power-on. Do not make changes to this register after initializing the host.

You can [`read`](crate::Reg::read) this register and get [`hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:HCFG)*/
pub struct HCFGrs;
impl crate::RegisterSpec for HCFGrs {
    type Ux = u32;
}
///`read()` method returns [`hcfg::R`](R) reader structure
impl crate::Readable for HCFGrs {}
///`write(|w| ..)` method takes [`hcfg::W`](W) writer structure
impl crate::Writable for HCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCFG to value 0
impl crate::Resettable for HCFGrs {}
