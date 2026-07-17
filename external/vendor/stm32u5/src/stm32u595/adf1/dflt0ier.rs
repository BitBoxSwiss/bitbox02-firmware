///Register `DFLT0IER` reader
pub type R = crate::R<DFLT0IERrs>;
///Register `DFLT0IER` writer
pub type W = crate::W<DFLT0IERrs>;
///Field `FTHIE` reader - RXFIFO threshold interrupt enable
pub type FTHIE_R = crate::BitReader;
///Field `FTHIE` writer - RXFIFO threshold interrupt enable
pub type FTHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOVRIE` reader - Data overflow interrupt enable
pub type DOVRIE_R = crate::BitReader;
///Field `DOVRIE` writer - Data overflow interrupt enable
pub type DOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATIE` reader - Saturation detection interrupt enable
pub type SATIE_R = crate::BitReader;
///Field `SATIE` writer - Saturation detection interrupt enable
pub type SATIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABIE` reader - Clock absence detection interrupt enable
pub type CKABIE_R = crate::BitReader;
///Field `CKABIE` writer - Clock absence detection interrupt enable
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFOVRIE` reader - Reshape filter overrun interrupt enable
pub type RFOVRIE_R = crate::BitReader;
///Field `RFOVRIE` writer - Reshape filter overrun interrupt enable
pub type RFOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDDETIE` reader - Sound activity detection interrupt enable
pub type SDDETIE_R = crate::BitReader;
///Field `SDDETIE` writer - Sound activity detection interrupt enable
pub type SDDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDLVLIE` reader - SAD sound-level value ready enable
pub type SDLVLIE_R = crate::BitReader;
///Field `SDLVLIE` writer - SAD sound-level value ready enable
pub type SDLVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow interrupt enable
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - Saturation detection interrupt enable
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection interrupt enable
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape filter overrun interrupt enable
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Sound activity detection interrupt enable
    #[inline(always)]
    pub fn sddetie(&self) -> SDDETIE_R {
        SDDETIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SAD sound-level value ready enable
    #[inline(always)]
    pub fn sdlvlie(&self) -> SDLVLIE_R {
        SDLVLIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0IER")
            .field("sdlvlie", &self.sdlvlie())
            .field("sddetie", &self.sddetie())
            .field("rfovrie", &self.rfovrie())
            .field("ckabie", &self.ckabie())
            .field("satie", &self.satie())
            .field("dovrie", &self.dovrie())
            .field("fthie", &self.fthie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn fthie(&mut self) -> FTHIE_W<DFLT0IERrs> {
        FTHIE_W::new(self, 0)
    }
    ///Bit 1 - Data overflow interrupt enable
    #[inline(always)]
    pub fn dovrie(&mut self) -> DOVRIE_W<DFLT0IERrs> {
        DOVRIE_W::new(self, 1)
    }
    ///Bit 9 - Saturation detection interrupt enable
    #[inline(always)]
    pub fn satie(&mut self) -> SATIE_W<DFLT0IERrs> {
        SATIE_W::new(self, 9)
    }
    ///Bit 10 - Clock absence detection interrupt enable
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<DFLT0IERrs> {
        CKABIE_W::new(self, 10)
    }
    ///Bit 11 - Reshape filter overrun interrupt enable
    #[inline(always)]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<DFLT0IERrs> {
        RFOVRIE_W::new(self, 11)
    }
    ///Bit 12 - Sound activity detection interrupt enable
    #[inline(always)]
    pub fn sddetie(&mut self) -> SDDETIE_W<DFLT0IERrs> {
        SDDETIE_W::new(self, 12)
    }
    ///Bit 13 - SAD sound-level value ready enable
    #[inline(always)]
    pub fn sdlvlie(&mut self) -> SDLVLIE_W<DFLT0IERrs> {
        SDLVLIE_W::new(self, 13)
    }
}
/**ADF DFLT0 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dflt0ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADF1:DFLT0IER)*/
pub struct DFLT0IERrs;
impl crate::RegisterSpec for DFLT0IERrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0ier::R`](R) reader structure
impl crate::Readable for DFLT0IERrs {}
///`write(|w| ..)` method takes [`dflt0ier::W`](W) writer structure
impl crate::Writable for DFLT0IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0IER to value 0
impl crate::Resettable for DFLT0IERrs {}
