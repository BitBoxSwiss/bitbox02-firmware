///Register `OCOLR_RGB565` reader
pub type R = crate::R<OCOLR_RGB565rs>;
///Register `OCOLR_RGB565` writer
pub type W = crate::W<OCOLR_RGB565rs>;
///Field `BLUE` reader - Blue value in RGB565 mode
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue value in RGB565 mode
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `GREEN` reader - Green value in RGB565 mode
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green value in RGB565 mode
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RED` reader - Red value in RGB565 mode
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red value in RGB565 mode
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Blue value in RGB565 mode
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:10 - Green value in RGB565 mode
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 5) & 0x3f) as u8)
    }
    ///Bits 11:15 - Red value in RGB565 mode
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCOLR_RGB565")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Blue value in RGB565 mode
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<OCOLR_RGB565rs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 5:10 - Green value in RGB565 mode
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<OCOLR_RGB565rs> {
        GREEN_W::new(self, 5)
    }
    ///Bits 11:15 - Red value in RGB565 mode
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<OCOLR_RGB565rs> {
        RED_W::new(self, 11)
    }
}
/**output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_rgb565::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_rgb565::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DMA2D:OCOLR_RGB565)*/
pub struct OCOLR_RGB565rs;
impl crate::RegisterSpec for OCOLR_RGB565rs {
    type Ux = u32;
}
///`read()` method returns [`ocolr_rgb565::R`](R) reader structure
impl crate::Readable for OCOLR_RGB565rs {}
///`write(|w| ..)` method takes [`ocolr_rgb565::W`](W) writer structure
impl crate::Writable for OCOLR_RGB565rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCOLR_RGB565 to value 0
impl crate::Resettable for OCOLR_RGB565rs {}
