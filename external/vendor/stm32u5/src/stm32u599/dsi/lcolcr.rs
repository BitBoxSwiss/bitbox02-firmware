///Register `LCOLCR` reader
pub type R = crate::R<LCOLCRrs>;
///Register `LCOLCR` writer
pub type W = crate::W<LCOLCRrs>;
///Field `COLC` reader - Color coding This field configures the DPI color coding. Others: Reserved
pub type COLC_R = crate::FieldReader;
///Field `COLC` writer - Color coding This field configures the DPI color coding. Others: Reserved
pub type COLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPE` reader - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
pub type LPE_R = crate::BitReader;
///Field `LPE` writer - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
pub type LPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Color coding This field configures the DPI color coding. Others: Reserved
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCOLCR")
            .field("colc", &self.colc())
            .field("lpe", &self.lpe())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Color coding This field configures the DPI color coding. Others: Reserved
    #[inline(always)]
    pub fn colc(&mut self) -> COLC_W<LCOLCRrs> {
        COLC_W::new(self, 0)
    }
    ///Bit 8 - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
    #[inline(always)]
    pub fn lpe(&mut self) -> LPE_W<LCOLCRrs> {
        LPE_W::new(self, 8)
    }
}
/**DSI Host LTDC color coding register

You can [`read`](crate::Reg::read) this register and get [`lcolcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcolcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:LCOLCR)*/
pub struct LCOLCRrs;
impl crate::RegisterSpec for LCOLCRrs {
    type Ux = u32;
}
///`read()` method returns [`lcolcr::R`](R) reader structure
impl crate::Readable for LCOLCRrs {}
///`write(|w| ..)` method takes [`lcolcr::W`](W) writer structure
impl crate::Writable for LCOLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCOLCR to value 0
impl crate::Resettable for LCOLCRrs {}
