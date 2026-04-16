///Register `OPAMP2_LPOTR` reader
pub type R = crate::R<OPAMP2_LPOTRrs>;
///Register `OPAMP2_LPOTR` writer
pub type W = crate::W<OPAMP2_LPOTRrs>;
///Field `TRIMLPOFFSETN` reader - Low-power mode trim for NMOS differential pairs
pub type TRIMLPOFFSETN_R = crate::FieldReader;
///Field `TRIMLPOFFSETN` writer - Low-power mode trim for NMOS differential pairs
pub type TRIMLPOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRIMLPOFFSETP` reader - Low-power mode trim for PMOS differential pairs
pub type TRIMLPOFFSETP_R = crate::FieldReader;
///Field `TRIMLPOFFSETP` writer - Low-power mode trim for PMOS differential pairs
pub type TRIMLPOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Low-power mode trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TRIMLPOFFSETN_R {
        TRIMLPOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Low-power mode trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TRIMLPOFFSETP_R {
        TRIMLPOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPAMP2_LPOTR")
            .field("trimlpoffsetn", &self.trimlpoffsetn())
            .field("trimlpoffsetp", &self.trimlpoffsetp())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Low-power mode trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W<OPAMP2_LPOTRrs> {
        TRIMLPOFFSETN_W::new(self, 0)
    }
    ///Bits 8:12 - Low-power mode trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W<OPAMP2_LPOTRrs> {
        TRIMLPOFFSETP_W::new(self, 8)
    }
}
/**OPAMP2 offset trimming register in low-power mode

You can [`read`](crate::Reg::read) this register and get [`opamp2_lpotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_lpotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OPAMP:OPAMP2_LPOTR)*/
pub struct OPAMP2_LPOTRrs;
impl crate::RegisterSpec for OPAMP2_LPOTRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp2_lpotr::R`](R) reader structure
impl crate::Readable for OPAMP2_LPOTRrs {}
///`write(|w| ..)` method takes [`opamp2_lpotr::W`](W) writer structure
impl crate::Writable for OPAMP2_LPOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP2_LPOTR to value 0
impl crate::Resettable for OPAMP2_LPOTRrs {}
