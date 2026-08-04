///Register `ADR` reader
pub type R = crate::R<ADRrs>;
///Register `ADR` writer
pub type W = crate::W<ADRrs>;
///Field `DATA` reader - Data
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - Data
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADR").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<ADRrs> {
        DATA_W::new(self, 0)
    }
}
/**A Data register

You can [`read`](crate::Reg::read) this register and get [`adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SAI1:ADR)*/
pub struct ADRrs;
impl crate::RegisterSpec for ADRrs {
    type Ux = u32;
}
///`read()` method returns [`adr::R`](R) reader structure
impl crate::Readable for ADRrs {}
///`write(|w| ..)` method takes [`adr::W`](W) writer structure
impl crate::Writable for ADRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADR to value 0
impl crate::Resettable for ADRrs {}
