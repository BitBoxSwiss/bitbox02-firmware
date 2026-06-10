///Register `ALRM%sSSR` reader
pub type R = crate::R<ALRMSSRrs>;
///Register `ALRM%sSSR` writer
pub type W = crate::W<ALRMSSRrs>;
///Field `SS` reader - Sub seconds value
pub type SS_R = crate::FieldReader<u16>;
///Field `SS` writer - Sub seconds value
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16, crate::Safe>;
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit
pub type MASKSS_R = crate::FieldReader;
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SSCLR` reader - SSCLR
pub type SSCLR_R = crate::BitReader;
///Field `SSCLR` writer - SSCLR
pub type SSCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 31 - SSCLR
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMSSR")
            .field("ssclr", &self.ssclr())
            .field("maskss", &self.maskss())
            .field("ss", &self.ss())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<ALRMSSRrs> {
        SS_W::new(self, 0)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W<ALRMSSRrs> {
        MASKSS_W::new(self, 24)
    }
    ///Bit 31 - SSCLR
    #[inline(always)]
    pub fn ssclr(&mut self) -> SSCLR_W<ALRMSSRrs> {
        SSCLR_W::new(self, 31)
    }
}
/**Alarm %s sub-second register

You can [`read`](crate::Reg::read) this register and get [`alrmssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RTC:ALRM[A]SSR)*/
pub struct ALRMSSRrs;
impl crate::RegisterSpec for ALRMSSRrs {
    type Ux = u32;
}
///`read()` method returns [`alrmssr::R`](R) reader structure
impl crate::Readable for ALRMSSRrs {}
///`write(|w| ..)` method takes [`alrmssr::W`](W) writer structure
impl crate::Writable for ALRMSSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRM%sSSR to value 0
impl crate::Resettable for ALRMSSRrs {}
