///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Register `IDR` writer
pub type W = crate::W<IDRrs>;
///Field `IDR` reader - General-purpose 8-bit data register bits
pub type IDR_R = crate::FieldReader<u32>;
///Field `IDR` writer - General-purpose 8-bit data register bits
pub type IDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - General-purpose 8-bit data register bits
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("idr", &self.idr()).finish()
    }
}
impl W {
    ///Bits 0:31 - General-purpose 8-bit data register bits
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W<IDRrs> {
        IDR_W::new(self, 0)
    }
}
/**Independent data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`write(|w| ..)` method takes [`idr::W`](W) writer structure
impl crate::Writable for IDRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
