///Register `OPAMP2_OTR` reader
pub type R = crate::R<OPAMP2_OTRrs>;
///Register `OPAMP2_OTR` writer
pub type W = crate::W<OPAMP2_OTRrs>;
///Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs
pub type TRIMOFFSETN_R = crate::FieldReader;
///Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs
pub type TRIMOFFSETP_R = crate::FieldReader;
///Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP2_OTR")
            .field("trimoffsetn", &self.trimoffsetn())
            .field("trimoffsetp", &self.trimoffsetp())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<OPAMP2_OTRrs> {
        TRIMOFFSETN_W::new(self, 0)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<OPAMP2_OTRrs> {
        TRIMOFFSETP_W::new(self, 8)
    }
}
/**OPAMP2 offset trimming register in normal mode

You can [`read`](crate::Reg::read) this register and get [`opamp2_otr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_otr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OPAMP:OPAMP2_OTR)*/
pub struct OPAMP2_OTRrs;
impl crate::RegisterSpec for OPAMP2_OTRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp2_otr::R`](R) reader structure
impl crate::Readable for OPAMP2_OTRrs {}
///`write(|w| ..)` method takes [`opamp2_otr::W`](W) writer structure
impl crate::Writable for OPAMP2_OTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP2_OTR to value 0
impl crate::Resettable for OPAMP2_OTRrs {}
