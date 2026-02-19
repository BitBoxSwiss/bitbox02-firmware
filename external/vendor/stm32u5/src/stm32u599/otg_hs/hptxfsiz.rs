///Register `HPTXFSIZ` reader
pub type R = crate::R<HPTXFSIZrs>;
///Register `HPTXFSIZ` writer
pub type W = crate::W<HPTXFSIZrs>;
///Field `PTXSA` reader - PTXSA
pub type PTXSA_R = crate::FieldReader<u16>;
///Field `PTXSA` writer - PTXSA
pub type PTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PTXFSIZ` reader - PTXFSIZ
pub type PTXFSIZ_R = crate::FieldReader<u16>;
///Field `PTXFSIZ` writer - PTXFSIZ
pub type PTXFSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PTXSA
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - PTXFSIZ
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXFSIZ")
            .field("ptxsa", &self.ptxsa())
            .field("ptxfsiz", &self.ptxfsiz())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - PTXSA
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W<HPTXFSIZrs> {
        PTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - PTXFSIZ
    #[inline(always)]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W<HPTXFSIZrs> {
        PTXFSIZ_W::new(self, 16)
    }
}
/**OTG host periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:HPTXFSIZ)*/
pub struct HPTXFSIZrs;
impl crate::RegisterSpec for HPTXFSIZrs {
    type Ux = u32;
}
///`read()` method returns [`hptxfsiz::R`](R) reader structure
impl crate::Readable for HPTXFSIZrs {}
///`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure
impl crate::Writable for HPTXFSIZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPTXFSIZ to value 0x0200_0800
impl crate::Resettable for HPTXFSIZrs {
    const RESET_VALUE: u32 = 0x0200_0800;
}
