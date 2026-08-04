///Register `HNPTXFSIZ` reader
pub type R = crate::R<HNPTXFSIZrs>;
///Register `HNPTXFSIZ` writer
pub type W = crate::W<HNPTXFSIZrs>;
///Field `NPTXFSA` reader - NPTXFSA
pub type NPTXFSA_R = crate::FieldReader<u16>;
///Field `NPTXFSA` writer - NPTXFSA
pub type NPTXFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `NPTXFD` reader - NPTXFD
pub type NPTXFD_R = crate::FieldReader<u16>;
///Field `NPTXFD` writer - NPTXFD
pub type NPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NPTXFSA
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - NPTXFD
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HNPTXFSIZ")
            .field("nptxfsa", &self.nptxfsa())
            .field("nptxfd", &self.nptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - NPTXFSA
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<HNPTXFSIZrs> {
        NPTXFSA_W::new(self, 0)
    }
    ///Bits 16:31 - NPTXFD
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W<HNPTXFSIZrs> {
        NPTXFD_W::new(self, 16)
    }
}
/**Host mode

You can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OTG_FS:HNPTXFSIZ)*/
pub struct HNPTXFSIZrs;
impl crate::RegisterSpec for HNPTXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`hnptxfsiz::R`](R) reader structure
impl crate::Readable for HNPTXFSIZrs {}
///`write(|w| ..)` method takes [`hnptxfsiz::W`](W) writer structure
impl crate::Writable for HNPTXFSIZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HNPTXFSIZ to value 0x0200_0200
impl crate::Resettable for HNPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0200;
}
