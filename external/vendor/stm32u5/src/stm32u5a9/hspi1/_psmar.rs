///Register `_PSMAR` reader
pub type R = crate::R<_PSMARrs>;
///Register `_PSMAR` writer
pub type W = crate::W<_PSMARrs>;
///Field `MATCH` reader - 31: 0\]: Status match Value to be compared with the masked status register to get a match
pub type MATCH_R = crate::FieldReader<u32>;
///Field `MATCH` writer - 31: 0\]: Status match Value to be compared with the masked status register to get a match
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 31: 0\]: Status match Value to be compared with the masked status register to get a match
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_PSMAR")
            .field("match_", &self.match_())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - 31: 0\]: Status match Value to be compared with the masked status register to get a match
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W<_PSMARrs> {
        MATCH_W::new(self, 0)
    }
}
/**HSPI polling status match register

You can [`read`](crate::Reg::read) this register and get [`_psmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_PSMAR)*/
pub struct _PSMARrs;
impl crate::RegisterSpec for _PSMARrs {
    type Ux = u32;
}
///`read()` method returns [`_psmar::R`](R) reader structure
impl crate::Readable for _PSMARrs {}
///`write(|w| ..)` method takes [`_psmar::W`](W) writer structure
impl crate::Writable for _PSMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _PSMAR to value 0
impl crate::Resettable for _PSMARrs {}
