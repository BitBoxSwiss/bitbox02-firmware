///Register `UCPDR` reader
pub type R = crate::R<UCPDRrs>;
///Register `UCPDR` writer
pub type W = crate::W<UCPDRrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**PWR USB Type-C™ and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:UCPDR)*/
pub struct UCPDRrs;
impl crate::RegisterSpec for UCPDRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpdr::R`](R) reader structure
impl crate::Readable for UCPDRrs {}
///`write(|w| ..)` method takes [`ucpdr::W`](W) writer structure
impl crate::Writable for UCPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UCPDR to value 0
impl crate::Resettable for UCPDRrs {}
