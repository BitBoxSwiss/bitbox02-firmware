///Register `DIEPTXF6` reader
pub type R = crate::R<DIEPTXF6rs>;
///Register `DIEPTXF6` writer
pub type W = crate::W<DIEPTXF6rs>;
///Field `INEPTXSA` reader - INEPTXSA
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - INEPTXSA
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - INEPTXFD
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - INEPTXFD
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - INEPTXSA
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - INEPTXFD
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF6")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - INEPTXSA
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<DIEPTXF6rs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - INEPTXFD
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<DIEPTXF6rs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG device IN endpoint transmit FIFO 6 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:DIEPTXF6)*/
pub struct DIEPTXF6rs;
impl crate::RegisterSpec for DIEPTXF6rs {
    type Ux = u32;
}
///`read()` method returns [`dieptxf6::R`](R) reader structure
impl crate::Readable for DIEPTXF6rs {}
///`write(|w| ..)` method takes [`dieptxf6::W`](W) writer structure
impl crate::Writable for DIEPTXF6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPTXF6 to value 0x0200_0e00
impl crate::Resettable for DIEPTXF6rs {
    const RESET_VALUE: u32 = 0x0200_0e00;
}
