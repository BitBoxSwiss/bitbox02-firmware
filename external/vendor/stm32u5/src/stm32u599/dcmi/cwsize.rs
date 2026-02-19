///Register `CWSIZE` reader
pub type R = crate::R<CWSIZErs>;
///Register `CWSIZE` writer
pub type W = crate::W<CWSIZErs>;
///Field `CAPCNT` reader - Capture count
pub type CAPCNT_R = crate::FieldReader<u16>;
///Field `CAPCNT` writer - Capture count
pub type CAPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
///Field `VLINE` reader - Vertical line count
pub type VLINE_R = crate::FieldReader<u16>;
///Field `VLINE` writer - Vertical line count
pub type VLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    ///Bits 0:13 - Capture count
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:29 - Vertical line count
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWSIZE")
            .field("vline", &self.vline())
            .field("capcnt", &self.capcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Capture count
    #[inline(always)]
    pub fn capcnt(&mut self) -> CAPCNT_W<CWSIZErs> {
        CAPCNT_W::new(self, 0)
    }
    ///Bits 16:29 - Vertical line count
    #[inline(always)]
    pub fn vline(&mut self) -> VLINE_W<CWSIZErs> {
        VLINE_W::new(self, 16)
    }
}
/**crop window size

You can [`read`](crate::Reg::read) this register and get [`cwsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DCMI:CWSIZE)*/
pub struct CWSIZErs;
impl crate::RegisterSpec for CWSIZErs {
    type Ux = u32;
}
///`read()` method returns [`cwsize::R`](R) reader structure
impl crate::Readable for CWSIZErs {}
///`write(|w| ..)` method takes [`cwsize::W`](W) writer structure
impl crate::Writable for CWSIZErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CWSIZE to value 0
impl crate::Resettable for CWSIZErs {}
