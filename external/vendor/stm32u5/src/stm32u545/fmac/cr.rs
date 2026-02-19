///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RIEN` reader - Enable read interrupt
pub type RIEN_R = crate::BitReader;
///Field `RIEN` writer - Enable read interrupt
pub type RIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIEN` reader - Enable write interrupt
pub type WIEN_R = crate::BitReader;
///Field `WIEN` writer - Enable write interrupt
pub type WIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVFLIEN` reader - Enable overflow error interrupts
pub type OVFLIEN_R = crate::BitReader;
///Field `OVFLIEN` writer - Enable overflow error interrupts
pub type OVFLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNFLIEN` reader - Enable underflow error interrupts
pub type UNFLIEN_R = crate::BitReader;
///Field `UNFLIEN` writer - Enable underflow error interrupts
pub type UNFLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATIEN` reader - Enable saturation error interrupts
pub type SATIEN_R = crate::BitReader;
///Field `SATIEN` writer - Enable saturation error interrupts
pub type SATIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAREN` reader - Enable DMA read channel requests
pub type DMAREN_R = crate::BitReader;
///Field `DMAREN` writer - Enable DMA read channel requests
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAWEN` reader - Enable DMA write channel requests
pub type DMAWEN_R = crate::BitReader;
///Field `DMAWEN` writer - Enable DMA write channel requests
pub type DMAWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLIPEN` reader - Enable clipping
pub type CLIPEN_R = crate::BitReader;
///Field `CLIPEN` writer - Enable clipping
pub type CLIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET` reader - Reset FMAC unit
pub type RESET_R = crate::BitReader;
///Field `RESET` writer - Reset FMAC unit
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable read interrupt
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable write interrupt
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable overflow error interrupts
    #[inline(always)]
    pub fn ovflien(&self) -> OVFLIEN_R {
        OVFLIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable underflow error interrupts
    #[inline(always)]
    pub fn unflien(&self) -> UNFLIEN_R {
        UNFLIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable saturation error interrupts
    #[inline(always)]
    pub fn satien(&self) -> SATIEN_R {
        SATIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Enable DMA read channel requests
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable DMA write channel requests
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Enable clipping
    #[inline(always)]
    pub fn clipen(&self) -> CLIPEN_R {
        CLIPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Reset FMAC unit
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("reset", &self.reset())
            .field("clipen", &self.clipen())
            .field("dmawen", &self.dmawen())
            .field("dmaren", &self.dmaren())
            .field("satien", &self.satien())
            .field("unflien", &self.unflien())
            .field("ovflien", &self.ovflien())
            .field("wien", &self.wien())
            .field("rien", &self.rien())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable read interrupt
    #[inline(always)]
    pub fn rien(&mut self) -> RIEN_W<CRrs> {
        RIEN_W::new(self, 0)
    }
    ///Bit 1 - Enable write interrupt
    #[inline(always)]
    pub fn wien(&mut self) -> WIEN_W<CRrs> {
        WIEN_W::new(self, 1)
    }
    ///Bit 2 - Enable overflow error interrupts
    #[inline(always)]
    pub fn ovflien(&mut self) -> OVFLIEN_W<CRrs> {
        OVFLIEN_W::new(self, 2)
    }
    ///Bit 3 - Enable underflow error interrupts
    #[inline(always)]
    pub fn unflien(&mut self) -> UNFLIEN_W<CRrs> {
        UNFLIEN_W::new(self, 3)
    }
    ///Bit 4 - Enable saturation error interrupts
    #[inline(always)]
    pub fn satien(&mut self) -> SATIEN_W<CRrs> {
        SATIEN_W::new(self, 4)
    }
    ///Bit 8 - Enable DMA read channel requests
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W<CRrs> {
        DMAREN_W::new(self, 8)
    }
    ///Bit 9 - Enable DMA write channel requests
    #[inline(always)]
    pub fn dmawen(&mut self) -> DMAWEN_W<CRrs> {
        DMAWEN_W::new(self, 9)
    }
    ///Bit 15 - Enable clipping
    #[inline(always)]
    pub fn clipen(&mut self) -> CLIPEN_W<CRrs> {
        CLIPEN_W::new(self, 15)
    }
    ///Bit 16 - Reset FMAC unit
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<CRrs> {
        RESET_W::new(self, 16)
    }
}
/**FMAC Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FMAC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
