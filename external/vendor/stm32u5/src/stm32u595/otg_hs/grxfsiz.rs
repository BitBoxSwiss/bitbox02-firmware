///Register `GRXFSIZ` reader
pub type R = crate::R<GRXFSIZrs>;
///Register `GRXFSIZ` writer
pub type W = crate::W<GRXFSIZrs>;
///Field `RXFD` reader - RXFD
pub type RXFD_R = crate::FieldReader<u16>;
///Field `RXFD` writer - RXFD
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - RXFD
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXFSIZ")
            .field("rxfd", &self.rxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - RXFD
    #[inline(always)]
    pub fn rxfd(&mut self) -> RXFD_W<GRXFSIZrs> {
        RXFD_W::new(self, 0)
    }
}
/**The application can program the RAM size that must be allocated to the Rx FIFO.

You can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:GRXFSIZ)*/
pub struct GRXFSIZrs;
impl crate::RegisterSpec for GRXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`grxfsiz::R`](R) reader structure
impl crate::Readable for GRXFSIZrs {}
///`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure
impl crate::Writable for GRXFSIZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GRXFSIZ to value 0x0400
impl crate::Resettable for GRXFSIZrs {
    const RESET_VALUE: u32 = 0x0400;
}
