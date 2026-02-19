///Register `GUSBCFG` reader
pub type R = crate::R<GUSBCFGrs>;
///Register `GUSBCFG` writer
pub type W = crate::W<GUSBCFGrs>;
///Field `TOCAL` reader - TOCAL
pub type TOCAL_R = crate::FieldReader;
///Field `TOCAL` writer - TOCAL
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PHYSEL` reader - PHYSEL
pub type PHYSEL_R = crate::BitReader;
///Field `SRPCAP` reader - SRPCAP
pub type SRPCAP_R = crate::BitReader;
///Field `SRPCAP` writer - SRPCAP
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNPCAP` reader - HNPCAP
pub type HNPCAP_R = crate::BitReader;
///Field `HNPCAP` writer - HNPCAP
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRDT` reader - TRDT
pub type TRDT_R = crate::FieldReader;
///Field `TRDT` writer - TRDT
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FHMOD` reader - FHMOD
pub type FHMOD_R = crate::BitReader;
///Field `FHMOD` writer - FHMOD
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDMOD` reader - FDMOD
pub type FDMOD_R = crate::BitReader;
///Field `FDMOD` writer - FDMOD
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - TOCAL
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    ///Bit 6 - PHYSEL
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - SRPCAP
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNPCAP
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:13 - TRDT
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 29 - FHMOD
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - FDMOD
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("tocal", &self.tocal())
            .field("physel", &self.physel())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("trdt", &self.trdt())
            .field("fhmod", &self.fhmod())
            .field("fdmod", &self.fdmod())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - TOCAL
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W<GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    ///Bit 8 - SRPCAP
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W<GUSBCFGrs> {
        SRPCAP_W::new(self, 8)
    }
    ///Bit 9 - HNPCAP
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W<GUSBCFGrs> {
        HNPCAP_W::new(self, 9)
    }
    ///Bits 10:13 - TRDT
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W<GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    ///Bit 29 - FHMOD
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W<GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    ///Bit 30 - FDMOD
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W<GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
}
/**This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.

You can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OTG_FS:GUSBCFG)*/
pub struct GUSBCFGrs;
impl crate::RegisterSpec for GUSBCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gusbcfg::R`](R) reader structure
impl crate::Readable for GUSBCFGrs {}
///`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure
impl crate::Writable for GUSBCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GUSBCFG to value 0x1440
impl crate::Resettable for GUSBCFGrs {
    const RESET_VALUE: u32 = 0x1440;
}
