///Register `CRCPOLY` reader
pub type R = crate::R<CRCPOLYrs>;
///Register `CRCPOLY` writer
pub type W = crate::W<CRCPOLYrs>;
///Field `CRCPOLY` reader - CRC polynomial register
pub type CRCPOLY_R = crate::FieldReader<u32>;
///Field `CRCPOLY` writer - CRC polynomial register
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - CRC polynomial register
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCPOLY")
            .field("crcpoly", &self.crcpoly())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRC polynomial register
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<CRCPOLYrs> {
        CRCPOLY_W::new(self, 0)
    }
}
/**Polynomial Register

You can [`read`](crate::Reg::read) this register and get [`crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#SPI1:CRCPOLY)*/
pub struct CRCPOLYrs;
impl crate::RegisterSpec for CRCPOLYrs {
    type Ux = u32;
}
///`read()` method returns [`crcpoly::R`](R) reader structure
impl crate::Readable for CRCPOLYrs {}
///`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure
impl crate::Writable for CRCPOLYrs {
    type Safety = crate::Safe;
}
///`reset()` method sets CRCPOLY to value 0x0107
impl crate::Resettable for CRCPOLYrs {
    const RESET_VALUE: u32 = 0x0107;
}
