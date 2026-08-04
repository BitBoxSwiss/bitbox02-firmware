///Register `UCPDR` reader
pub type R = crate::R<UCPDRrs>;
///Register `UCPDR` writer
pub type W = crate::W<UCPDRrs>;
///Field `CC1ENRXFILTER` reader - CC1ENRXFILTER
pub type CC1ENRXFILTER_R = crate::BitReader;
///Field `CC1ENRXFILTER` writer - CC1ENRXFILTER
pub type CC1ENRXFILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2ENRXFILTER` reader - CC2ENRXFILTER
pub type CC2ENRXFILTER_R = crate::BitReader;
///Field `CC2ENRXFILTER` writer - CC2ENRXFILTER
pub type CC2ENRXFILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CC1ENRXFILTER
    #[inline(always)]
    pub fn cc1enrxfilter(&self) -> CC1ENRXFILTER_R {
        CC1ENRXFILTER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC2ENRXFILTER
    #[inline(always)]
    pub fn cc2enrxfilter(&self) -> CC2ENRXFILTER_R {
        CC2ENRXFILTER_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPDR")
            .field("cc1enrxfilter", &self.cc1enrxfilter())
            .field("cc2enrxfilter", &self.cc2enrxfilter())
            .finish()
    }
}
impl W {
    ///Bit 0 - CC1ENRXFILTER
    #[inline(always)]
    pub fn cc1enrxfilter(&mut self) -> CC1ENRXFILTER_W<UCPDRrs> {
        CC1ENRXFILTER_W::new(self, 0)
    }
    ///Bit 1 - CC2ENRXFILTER
    #[inline(always)]
    pub fn cc2enrxfilter(&mut self) -> CC2ENRXFILTER_W<UCPDRrs> {
        CC2ENRXFILTER_W::new(self, 1)
    }
}
/**USB Type C and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SYSCFG:UCPDR)*/
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
