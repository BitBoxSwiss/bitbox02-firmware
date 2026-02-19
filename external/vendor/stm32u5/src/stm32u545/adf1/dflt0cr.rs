///Register `DFLT0CR` reader
pub type R = crate::R<DFLT0CRrs>;
///Register `DFLT0CR` writer
pub type W = crate::W<DFLT0CRrs>;
///Field `DFLTEN` reader - DFLT0 enable
pub type DFLTEN_R = crate::BitReader;
///Field `DFLTEN` writer - DFLT0 enable
pub type DFLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTH` reader - RXFIFO threshold selection
pub type FTH_R = crate::BitReader;
///Field `FTH` writer - RXFIFO threshold selection
pub type FTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACQMOD` reader - DFLT0 trigger mode
pub type ACQMOD_R = crate::FieldReader;
///Field `ACQMOD` writer - DFLT0 trigger mode
pub type ACQMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRGSRC` reader - DFLT0 trigger signal selection
pub type TRGSRC_R = crate::FieldReader;
///Field `TRGSRC` writer - DFLT0 trigger signal selection
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NBDIS` reader - Number of samples to be discarded
pub type NBDIS_R = crate::FieldReader;
///Field `NBDIS` writer - Number of samples to be discarded
pub type NBDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DFLTRUN` reader - DFLT0 run status flag
pub type DFLTRUN_R = crate::BitReader;
///Field `DFLTRUN` writer - DFLT0 run status flag
pub type DFLTRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFLTACTIVE` reader - DFLT0 active flag
pub type DFLTACTIVE_R = crate::BitReader;
///Field `DFLTACTIVE` writer - DFLT0 active flag
pub type DFLTACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFLT0 enable
    #[inline(always)]
    pub fn dflten(&self) -> DFLTEN_R {
        DFLTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - DFLT0 trigger mode
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - DFLT0 run status flag
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DFLT0 active flag
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0CR")
            .field("dfltactive", &self.dfltactive())
            .field("dfltrun", &self.dfltrun())
            .field("nbdis", &self.nbdis())
            .field("trgsrc", &self.trgsrc())
            .field("acqmod", &self.acqmod())
            .field("fth", &self.fth())
            .field("dmaen", &self.dmaen())
            .field("dflten", &self.dflten())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFLT0 enable
    #[inline(always)]
    pub fn dflten(&mut self) -> DFLTEN_W<DFLT0CRrs> {
        DFLTEN_W::new(self, 0)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<DFLT0CRrs> {
        DMAEN_W::new(self, 1)
    }
    ///Bit 2 - RXFIFO threshold selection
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<DFLT0CRrs> {
        FTH_W::new(self, 2)
    }
    ///Bits 4:6 - DFLT0 trigger mode
    #[inline(always)]
    pub fn acqmod(&mut self) -> ACQMOD_W<DFLT0CRrs> {
        ACQMOD_W::new(self, 4)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W<DFLT0CRrs> {
        TRGSRC_W::new(self, 12)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    pub fn nbdis(&mut self) -> NBDIS_W<DFLT0CRrs> {
        NBDIS_W::new(self, 20)
    }
    ///Bit 30 - DFLT0 run status flag
    #[inline(always)]
    pub fn dfltrun(&mut self) -> DFLTRUN_W<DFLT0CRrs> {
        DFLTRUN_W::new(self, 30)
    }
    ///Bit 31 - DFLT0 active flag
    #[inline(always)]
    pub fn dfltactive(&mut self) -> DFLTACTIVE_W<DFLT0CRrs> {
        DFLTACTIVE_W::new(self, 31)
    }
}
/**ADF digital filter control register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADF1:DFLT0CR)*/
pub struct DFLT0CRrs;
impl crate::RegisterSpec for DFLT0CRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0cr::R`](R) reader structure
impl crate::Readable for DFLT0CRrs {}
///`write(|w| ..)` method takes [`dflt0cr::W`](W) writer structure
impl crate::Writable for DFLT0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0CR to value 0
impl crate::Resettable for DFLT0CRrs {}
