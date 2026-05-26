///Register `BGCLUT` reader
pub type R = crate::R<BGCLUTrs>;
///Register `BGCLUT` writer
pub type W = crate::W<BGCLUTrs>;
///Field `BLUE` reader - BLUE
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - BLUE
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` reader - GREEN
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - GREEN
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` reader - RED
pub type RED_R = crate::FieldReader;
///Field `RED` writer - RED
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `APLHA` reader - APLHA
pub type APLHA_R = crate::FieldReader;
///Field `APLHA` writer - APLHA
pub type APLHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - BLUE
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - GREEN
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RED
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - APLHA
    #[inline(always)]
    pub fn aplha(&self) -> APLHA_R {
        APLHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BGCLUT")
            .field("aplha", &self.aplha())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - BLUE
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<BGCLUTrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - GREEN
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<BGCLUTrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - RED
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<BGCLUTrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - APLHA
    #[inline(always)]
    pub fn aplha(&mut self) -> APLHA_W<BGCLUTrs> {
        APLHA_W::new(self, 24)
    }
}
/**BGCLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DMA2D:BGCLUT)*/
pub struct BGCLUTrs;
impl crate::RegisterSpec for BGCLUTrs {
    type Ux = u32;
}
///`read()` method returns [`bgclut::R`](R) reader structure
impl crate::Readable for BGCLUTrs {}
///`write(|w| ..)` method takes [`bgclut::W`](W) writer structure
impl crate::Writable for BGCLUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGCLUT to value 0
impl crate::Resettable for BGCLUTrs {}
