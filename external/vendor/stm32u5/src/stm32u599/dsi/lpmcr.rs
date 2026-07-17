///Register `LPMCR` reader
pub type R = crate::R<LPMCRrs>;
///Register `LPMCR` writer
pub type W = crate::W<LPMCRrs>;
///Field `VLPSIZE` reader - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions.
pub type VLPSIZE_R = crate::FieldReader;
///Field `VLPSIZE` writer - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions.
pub type VLPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LPSIZE` reader - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions.
pub type LPSIZE_R = crate::FieldReader;
///Field `LPSIZE` writer - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions.
pub type LPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions.
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions.
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMCR")
            .field("vlpsize", &self.vlpsize())
            .field("lpsize", &self.lpsize())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions.
    #[inline(always)]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<LPMCRrs> {
        VLPSIZE_W::new(self, 0)
    }
    ///Bits 16:23 - Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions.
    #[inline(always)]
    pub fn lpsize(&mut self) -> LPSIZE_W<LPMCRrs> {
        LPSIZE_W::new(self, 16)
    }
}
/**DSI Host low-power mode configuration register

You can [`read`](crate::Reg::read) this register and get [`lpmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:LPMCR)*/
pub struct LPMCRrs;
impl crate::RegisterSpec for LPMCRrs {
    type Ux = u32;
}
///`read()` method returns [`lpmcr::R`](R) reader structure
impl crate::Readable for LPMCRrs {}
///`write(|w| ..)` method takes [`lpmcr::W`](W) writer structure
impl crate::Writable for LPMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPMCR to value 0
impl crate::Resettable for LPMCRrs {}
