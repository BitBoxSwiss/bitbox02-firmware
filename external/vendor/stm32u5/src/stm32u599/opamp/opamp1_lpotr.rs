///Register `OPAMP1_LPOTR` reader
pub type R = crate::R<OPAMP1_LPOTRrs>;
///Register `OPAMP1_LPOTR` writer
pub type W = crate::W<OPAMP1_LPOTRrs>;
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
        f.debug_struct("OPAMP1_LPOTR")
            .field("trimlpoffsetn", &self.trimlpoffsetn())
            .field("trimlpoffsetp", &self.trimlpoffsetp())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Low-power mode trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W<OPAMP1_LPOTRrs> {
        TRIMLPOFFSETN_W::new(self, 0)
    }
    ///Bits 8:12 - Low-power mode trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W<OPAMP1_LPOTRrs> {
        TRIMLPOFFSETP_W::new(self, 8)
    }
}
/**OPAMP1 offset trimming register in low-power mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_lpotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_lpotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OPAMP:OPAMP1_LPOTR)*/
pub struct OPAMP1_LPOTRrs;
impl crate::RegisterSpec for OPAMP1_LPOTRrs {
    type Ux = u32;
}
///`read()` method returns [`opamp1_lpotr::R`](R) reader structure
impl crate::Readable for OPAMP1_LPOTRrs {}
///`write(|w| ..)` method takes [`opamp1_lpotr::W`](W) writer structure
impl crate::Writable for OPAMP1_LPOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPAMP1_LPOTR to value 0
impl crate::Resettable for OPAMP1_LPOTRrs {}
