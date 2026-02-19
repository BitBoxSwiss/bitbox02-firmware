///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
///Field `TCDMAEN` reader - DMA request enable on Transfer Complete event
pub type TCDMAEN_R = crate::BitReader;
///Field `TCDMAEN` writer - DMA request enable on Transfer Complete event
pub type TCDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCRDMAEN` reader - DMA request enable on Transfer Complete Reload event
pub type TCRDMAEN_R = crate::BitReader;
///Field `TCRDMAEN` writer - DMA request enable on Transfer Complete Reload event
pub type TCRDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGSEL` reader - Trigger selection
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - Trigger selection
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIGPOL` reader - Trigger polarity
pub type TRIGPOL_R = crate::BitReader;
///Field `TRIGPOL` writer - Trigger polarity
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGEN` reader - Trigger enable
pub type TRIGEN_R = crate::BitReader;
///Field `TRIGEN` writer - Trigger enable
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - DMA request enable on Transfer Complete event
    #[inline(always)]
    pub fn tcdmaen(&self) -> TCDMAEN_R {
        TCDMAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA request enable on Transfer Complete Reload event
    #[inline(always)]
    pub fn tcrdmaen(&self) -> TCRDMAEN_R {
        TCRDMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:19 - Trigger selection
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - Trigger polarity
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Trigger enable
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("tcdmaen", &self.tcdmaen())
            .field("tcrdmaen", &self.tcrdmaen())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("trigen", &self.trigen())
            .finish()
    }
}
impl W {
    ///Bit 6 - DMA request enable on Transfer Complete event
    #[inline(always)]
    pub fn tcdmaen(&mut self) -> TCDMAEN_W<AUTOCRrs> {
        TCDMAEN_W::new(self, 6)
    }
    ///Bit 7 - DMA request enable on Transfer Complete Reload event
    #[inline(always)]
    pub fn tcrdmaen(&mut self) -> TCRDMAEN_W<AUTOCRrs> {
        TCRDMAEN_W::new(self, 7)
    }
    ///Bits 16:19 - Trigger selection
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<AUTOCRrs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bit 20 - Trigger polarity
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<AUTOCRrs> {
        TRIGPOL_W::new(self, 20)
    }
    ///Bit 21 - Trigger enable
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<AUTOCRrs> {
        TRIGEN_W::new(self, 21)
    }
}
/**I2C Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#I2C1:AUTOCR)*/
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
///`read()` method returns [`autocr::R`](R) reader structure
impl crate::Readable for AUTOCRrs {}
///`write(|w| ..)` method takes [`autocr::W`](W) writer structure
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AUTOCR to value 0
impl crate::Resettable for AUTOCRrs {}
