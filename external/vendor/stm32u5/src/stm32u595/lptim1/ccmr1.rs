///Register `CCMR1` reader
pub type R = crate::R<CCMR1rs>;
///Register `CCMR1` writer
pub type W = crate::W<CCMR1rs>;
///Field `CC1SEL` reader - Capture/compare 1 selection
pub type CC1SEL_R = crate::BitReader;
///Field `CC1SEL` writer - Capture/compare 1 selection
pub type CC1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1E` reader - Capture/compare 1 output enable
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - Capture/compare 1 output enable
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - Capture/compare 1 output polarity
pub type CC1P_R = crate::FieldReader;
///Field `CC1P` writer - Capture/compare 1 output polarity
pub type CC1P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1PSC` reader - Input capture 1 prescaler
pub type IC1PSC_R = crate::FieldReader;
///Field `IC1PSC` writer - Input capture 1 prescaler
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC1F` reader - Input capture 1 filter
pub type IC1F_R = crate::FieldReader;
///Field `IC1F` writer - Input capture 1 filter
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CC2SEL` reader - Capture/compare 2 selection
pub type CC2SEL_R = crate::BitReader;
///Field `CC2SEL` writer - Capture/compare 2 selection
pub type CC2SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2E` reader - Capture/compare 2 output enable
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - Capture/compare 2 output enable
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - Capture/compare 2 output polarity
pub type CC2P_R = crate::FieldReader;
///Field `CC2P` writer - Capture/compare 2 output polarity
pub type CC2P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2PSC` reader - Input capture 2 prescaler
pub type IC2PSC_R = crate::FieldReader;
///Field `IC2PSC` writer - Input capture 2 prescaler
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2F` reader - Input capture 2 filter
pub type IC2F_R = crate::FieldReader;
///Field `IC2F` writer - Input capture 2 filter
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Capture/compare 1 selection
    #[inline(always)]
    pub fn cc1sel(&self) -> CC1SEL_R {
        CC1SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Capture/compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 8:9 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 16 - Capture/compare 2 selection
    #[inline(always)]
    pub fn cc2sel(&self) -> CC2SEL_R {
        CC2SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Capture/compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Capture/compare 2 output polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 24:25 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1")
            .field("cc1sel", &self.cc1sel())
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .field("cc2sel", &self.cc2sel())
            .field("cc2e", &self.cc2e())
            .field("cc2p", &self.cc2p())
            .field("ic2psc", &self.ic2psc())
            .field("ic2f", &self.ic2f())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 selection
    #[inline(always)]
    pub fn cc1sel(&mut self) -> CC1SEL_W<CCMR1rs> {
        CC1SEL_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<CCMR1rs> {
        CC1E_W::new(self, 1)
    }
    ///Bits 2:3 - Capture/compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<CCMR1rs> {
        CC1P_W::new(self, 2)
    }
    ///Bits 8:9 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<CCMR1rs> {
        IC1PSC_W::new(self, 8)
    }
    ///Bits 12:13 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<CCMR1rs> {
        IC1F_W::new(self, 12)
    }
    ///Bit 16 - Capture/compare 2 selection
    #[inline(always)]
    pub fn cc2sel(&mut self) -> CC2SEL_W<CCMR1rs> {
        CC2SEL_W::new(self, 16)
    }
    ///Bit 17 - Capture/compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<CCMR1rs> {
        CC2E_W::new(self, 17)
    }
    ///Bits 18:19 - Capture/compare 2 output polarity
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<CCMR1rs> {
        CC2P_W::new(self, 18)
    }
    ///Bits 24:25 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W<CCMR1rs> {
        IC2PSC_W::new(self, 24)
    }
    ///Bits 28:29 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W<CCMR1rs> {
        IC2F_W::new(self, 28)
    }
}
/**LPTIM capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPTIM1:CCMR1)*/
pub struct CCMR1rs;
impl crate::RegisterSpec for CCMR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1::R`](R) reader structure
impl crate::Readable for CCMR1rs {}
///`write(|w| ..)` method takes [`ccmr1::W`](W) writer structure
impl crate::Writable for CCMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR1 to value 0
impl crate::Resettable for CCMR1rs {}
