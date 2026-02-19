///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
///Field `TDN` reader - TDN
pub type TDN_R = crate::FieldReader<u16>;
///Field `TDN` writer - TDN
pub type TDN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TRIGPOL` reader - TRIPOL
pub type TRIGPOL_R = crate::BitReader;
///Field `TRIGPOL` writer - TRIPOL
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGEN` reader - TRIGEN
pub type TRIGEN_R = crate::BitReader;
///Field `TRIGEN` writer - TRIGEN
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEDIS` reader - IDLEDIS
pub type IDLEDIS_R = crate::BitReader;
///Field `IDLEDIS` writer - IDLEDIS
pub type IDLEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGSEL` reader - TRIGSEL
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - TRIGSEL
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TECLREN` reader - TECLREN
pub type TECLREN_R = crate::BitReader;
///Field `TECLREN` writer - TECLREN
pub type TECLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - TDN
    #[inline(always)]
    pub fn tdn(&self) -> TDN_R {
        TDN_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - TRIPOL
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TRIGEN
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IDLEDIS
    #[inline(always)]
    pub fn idledis(&self) -> IDLEDIS_R {
        IDLEDIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 31 - TECLREN
    #[inline(always)]
    pub fn teclren(&self) -> TECLREN_R {
        TECLREN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("teclren", &self.teclren())
            .field("idledis", &self.idledis())
            .field("trigsel", &self.trigsel())
            .field("trigen", &self.trigen())
            .field("trigpol", &self.trigpol())
            .field("tdn", &self.tdn())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - TDN
    #[inline(always)]
    pub fn tdn(&mut self) -> TDN_W<AUTOCRrs> {
        TDN_W::new(self, 0)
    }
    ///Bit 16 - TRIPOL
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<AUTOCRrs> {
        TRIGPOL_W::new(self, 16)
    }
    ///Bit 17 - TRIGEN
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<AUTOCRrs> {
        TRIGEN_W::new(self, 17)
    }
    ///Bit 18 - IDLEDIS
    #[inline(always)]
    pub fn idledis(&mut self) -> IDLEDIS_W<AUTOCRrs> {
        IDLEDIS_W::new(self, 18)
    }
    ///Bits 19:22 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<AUTOCRrs> {
        TRIGSEL_W::new(self, 19)
    }
    ///Bit 31 - TECLREN
    #[inline(always)]
    pub fn teclren(&mut self) -> TECLREN_W<AUTOCRrs> {
        TECLREN_W::new(self, 31)
    }
}
/**AUTOCR

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#USART1:AUTOCR)*/
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
///`reset()` method sets AUTOCR to value 0x8000_0000
impl crate::Resettable for AUTOCRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
