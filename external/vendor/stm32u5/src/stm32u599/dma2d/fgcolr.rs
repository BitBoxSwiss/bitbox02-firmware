///Register `FGCOLR` reader
pub type R = crate::R<FGCOLRrs>;
///Register `FGCOLR` writer
pub type W = crate::W<FGCOLRrs>;
///Field `BLUE` reader - Blue Value
pub type BLUE_R = crate::FieldReader;
///Field `BLUE` writer - Blue Value
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `GREEN` reader - Green Value
pub type GREEN_R = crate::FieldReader;
///Field `GREEN` writer - Green Value
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `RED` reader - Red Value
pub type RED_R = crate::FieldReader;
///Field `RED` writer - Red Value
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FGCOLR")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Blue Value
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<FGCOLRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green Value
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<FGCOLRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red Value
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<FGCOLRrs> {
        RED_W::new(self, 16)
    }
}
/**foreground color register

You can [`read`](crate::Reg::read) this register and get [`fgcolr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcolr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DMA2D:FGCOLR)*/
pub struct FGCOLRrs;
impl crate::RegisterSpec for FGCOLRrs {
    type Ux = u32;
}
///`read()` method returns [`fgcolr::R`](R) reader structure
impl crate::Readable for FGCOLRrs {}
///`write(|w| ..)` method takes [`fgcolr::W`](W) writer structure
impl crate::Writable for FGCOLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FGCOLR to value 0
impl crate::Resettable for FGCOLRrs {}
