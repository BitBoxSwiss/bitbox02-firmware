///Register `OCOLR_ARGB4444` reader
pub type R = crate::R<OCOLR_ARGB4444rs>;
///Register `OCOLR_ARGB4444` writer
pub type W = crate::W<OCOLR_ARGB4444rs>;
///Field `BLUE` reader - Blue value in ARGB4444 mode
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue value in ARGB4444 mode
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GREEN` reader - Green value in ARGB4444 mode
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green value in ARGB4444 mode
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RED` reader - Red value in ARGB4444 mode
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red value in ARGB4444 mode
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ALPHA` reader - Alpha channel value in ARGB4444
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Alpha channel value in ARGB4444
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Blue value in ARGB4444 mode
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Green value in ARGB4444 mode
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Red value in ARGB4444 mode
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alpha channel value in ARGB4444
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCOLR_ARGB4444")
            .field("alpha", &self.alpha())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Blue value in ARGB4444 mode
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<OCOLR_ARGB4444rs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 4:7 - Green value in ARGB4444 mode
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<OCOLR_ARGB4444rs> {
        GREEN_W::new(self, 4)
    }
    ///Bits 8:11 - Red value in ARGB4444 mode
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<OCOLR_ARGB4444rs> {
        RED_W::new(self, 8)
    }
    ///Bits 12:15 - Alpha channel value in ARGB4444
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<OCOLR_ARGB4444rs> {
        ALPHA_W::new(self, 12)
    }
}
/**output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_argb4444::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_argb4444::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DMA2D:OCOLR_ARGB4444)*/
pub struct OCOLR_ARGB4444rs;
impl crate::RegisterSpec for OCOLR_ARGB4444rs {
    type Ux = u32;
}
///`read()` method returns [`ocolr_argb4444::R`](R) reader structure
impl crate::Readable for OCOLR_ARGB4444rs {}
///`write(|w| ..)` method takes [`ocolr_argb4444::W`](W) writer structure
impl crate::Writable for OCOLR_ARGB4444rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCOLR_ARGB4444 to value 0
impl crate::Resettable for OCOLR_ARGB4444rs {}
