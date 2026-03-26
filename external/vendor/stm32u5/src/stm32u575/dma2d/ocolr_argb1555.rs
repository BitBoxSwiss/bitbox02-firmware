///Register `OCOLR_ARGB1555` reader
pub type R = crate::R<OCOLR_ARGB1555rs>;
///Register `OCOLR_ARGB1555` writer
pub type W = crate::W<OCOLR_ARGB1555rs>;
///Field `BLUE` reader - Blue value in ARGB1555 mode
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue value in ARGB1555 mode
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `GREEN` reader - Green value in ARGB1555 mode
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green value in ARGB1555 mode
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RED` reader - Red value in ARGB1555 mode
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red value in ARGB1555 mode
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `A` reader - Alpha channel value in ARGB1555 mode
pub type A_R = crate::BitReader;
///Field `A` writer - Alpha channel value in ARGB1555 mode
pub type A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Blue value in ARGB1555 mode
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - Green value in ARGB1555 mode
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - Red value in ARGB1555 mode
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - Alpha channel value in ARGB1555 mode
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCOLR_ARGB1555")
            .field("a", &self.a())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Blue value in ARGB1555 mode
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<OCOLR_ARGB1555rs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 5:9 - Green value in ARGB1555 mode
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<OCOLR_ARGB1555rs> {
        GREEN_W::new(self, 5)
    }
    ///Bits 10:14 - Red value in ARGB1555 mode
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<OCOLR_ARGB1555rs> {
        RED_W::new(self, 10)
    }
    ///Bit 15 - Alpha channel value in ARGB1555 mode
    #[inline(always)]
    pub fn a(&mut self) -> A_W<OCOLR_ARGB1555rs> {
        A_W::new(self, 15)
    }
}
/**output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_argb1555::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_argb1555::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DMA2D:OCOLR_ARGB1555)*/
pub struct OCOLR_ARGB1555rs;
impl crate::RegisterSpec for OCOLR_ARGB1555rs {
    type Ux = u32;
}
///`read()` method returns [`ocolr_argb1555::R`](R) reader structure
impl crate::Readable for OCOLR_ARGB1555rs {}
///`write(|w| ..)` method takes [`ocolr_argb1555::W`](W) writer structure
impl crate::Writable for OCOLR_ARGB1555rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCOLR_ARGB1555 to value 0
impl crate::Resettable for OCOLR_ARGB1555rs {}
