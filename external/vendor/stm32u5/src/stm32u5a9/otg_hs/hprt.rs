///Register `HPRT` reader
pub type R = crate::R<HPRTrs>;
///Register `HPRT` writer
pub type W = crate::W<HPRTrs>;
///Field `PCSTS` reader - PCSTS
pub type PCSTS_R = crate::BitReader;
///Field `PCDET` reader - PCDET
pub type PCDET_R = crate::BitReader;
///Field `PCDET` writer - PCDET
pub type PCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PENA` reader - PENA
pub type PENA_R = crate::BitReader;
///Field `PENA` writer - PENA
pub type PENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PENCHNG` reader - PENCHNG
pub type PENCHNG_R = crate::BitReader;
///Field `PENCHNG` writer - PENCHNG
pub type PENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POCA` reader - POCA
pub type POCA_R = crate::BitReader;
///Field `POCCHNG` reader - POCCHNG
pub type POCCHNG_R = crate::BitReader;
///Field `POCCHNG` writer - POCCHNG
pub type POCCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRES` reader - PRES
pub type PRES_R = crate::BitReader;
///Field `PRES` writer - PRES
pub type PRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSUSP` reader - PSUSP
pub type PSUSP_R = crate::BitReader;
///Field `PSUSP` writer - PSUSP
pub type PSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRST` reader - PRST
pub type PRST_R = crate::BitReader;
///Field `PRST` writer - PRST
pub type PRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLSTS` reader - PLSTS
pub type PLSTS_R = crate::FieldReader;
///Field `PPWR` reader - PPWR
pub type PPWR_R = crate::BitReader;
///Field `PPWR` writer - PPWR
pub type PPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTCTL` reader - PTCTL
pub type PTCTL_R = crate::FieldReader;
///Field `PTCTL` writer - PTCTL
pub type PTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PSPD` reader - PSPD
pub type PSPD_R = crate::FieldReader;
impl R {
    ///Bit 0 - PCSTS
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PCDET
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PENA
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PENCHNG
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - POCA
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - POCCHNG
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRES
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PSUSP
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PRST
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:11 - PLSTS
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - PPWR
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - PTCTL
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:18 - PSPD
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("pcsts", &self.pcsts())
            .field("pcdet", &self.pcdet())
            .field("pena", &self.pena())
            .field("penchng", &self.penchng())
            .field("poca", &self.poca())
            .field("pocchng", &self.pocchng())
            .field("pres", &self.pres())
            .field("psusp", &self.psusp())
            .field("prst", &self.prst())
            .field("plsts", &self.plsts())
            .field("ppwr", &self.ppwr())
            .field("ptctl", &self.ptctl())
            .field("pspd", &self.pspd())
            .finish()
    }
}
impl W {
    ///Bit 1 - PCDET
    #[inline(always)]
    pub fn pcdet(&mut self) -> PCDET_W<HPRTrs> {
        PCDET_W::new(self, 1)
    }
    ///Bit 2 - PENA
    #[inline(always)]
    pub fn pena(&mut self) -> PENA_W<HPRTrs> {
        PENA_W::new(self, 2)
    }
    ///Bit 3 - PENCHNG
    #[inline(always)]
    pub fn penchng(&mut self) -> PENCHNG_W<HPRTrs> {
        PENCHNG_W::new(self, 3)
    }
    ///Bit 5 - POCCHNG
    #[inline(always)]
    pub fn pocchng(&mut self) -> POCCHNG_W<HPRTrs> {
        POCCHNG_W::new(self, 5)
    }
    ///Bit 6 - PRES
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<HPRTrs> {
        PRES_W::new(self, 6)
    }
    ///Bit 7 - PSUSP
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W<HPRTrs> {
        PSUSP_W::new(self, 7)
    }
    ///Bit 8 - PRST
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W<HPRTrs> {
        PRST_W::new(self, 8)
    }
    ///Bit 12 - PPWR
    #[inline(always)]
    pub fn ppwr(&mut self) -> PPWR_W<HPRTrs> {
        PPWR_W::new(self, 12)
    }
    ///Bits 13:16 - PTCTL
    #[inline(always)]
    pub fn ptctl(&mut self) -> PTCTL_W<HPRTrs> {
        PTCTL_W::new(self, 13)
    }
}
/**This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.

You can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:HPRT)*/
pub struct HPRTrs;
impl crate::RegisterSpec for HPRTrs {
    type Ux = u32;
}
///`read()` method returns [`hprt::R`](R) reader structure
impl crate::Readable for HPRTrs {}
///`write(|w| ..)` method takes [`hprt::W`](W) writer structure
impl crate::Writable for HPRTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPRT to value 0
impl crate::Resettable for HPRTrs {}
