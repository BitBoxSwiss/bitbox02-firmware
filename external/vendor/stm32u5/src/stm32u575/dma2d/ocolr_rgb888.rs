///Register `OCOLR_RGB888` reader
pub type R = crate::R<OCOLR_RGB888rs>;
///Register `OCOLR_RGB888` writer
pub type W = crate::W<OCOLR_RGB888rs>;
///Field `BLUE` reader - Blue Value
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue Value
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` reader - Green Value
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green Value
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` reader - Red Value
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red Value
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `APLHA` reader - Alpha Channel Value
pub type APLHA_R = crate::FieldReader;
///Field `APLHA` writer - Alpha Channel Value
pub type APLHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Blue Value
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green Value
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red Value
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Alpha Channel Value
    #[inline(always)]
    pub fn aplha(&self) -> APLHA_R {
        APLHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCOLR_RGB888")
            .field("aplha", &self.aplha())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Blue Value
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<OCOLR_RGB888rs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green Value
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<OCOLR_RGB888rs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red Value
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<OCOLR_RGB888rs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha Channel Value
    #[inline(always)]
    pub fn aplha(&mut self) -> APLHA_W<OCOLR_RGB888rs> {
        APLHA_W::new(self, 24)
    }
}
/**output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_rgb888::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_rgb888::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DMA2D:OCOLR_RGB888)*/
pub struct OCOLR_RGB888rs;
impl crate::RegisterSpec for OCOLR_RGB888rs {
    type Ux = u32;
}
///`read()` method returns [`ocolr_rgb888::R`](R) reader structure
impl crate::Readable for OCOLR_RGB888rs {}
///`write(|w| ..)` method takes [`ocolr_rgb888::W`](W) writer structure
impl crate::Writable for OCOLR_RGB888rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCOLR_RGB888 to value 0
impl crate::Resettable for OCOLR_RGB888rs {}
