///Register `SECWM2R2` reader
pub type R = crate::R<SECWM2R2rs>;
///Register `SECWM2R2` writer
pub type W = crate::W<SECWM2R2rs>;
///Field `PCROP2_PSTRT` reader - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2.
pub type PCROP2_PSTRT_R = crate::FieldReader;
///Field `PCROP2_PSTRT` writer - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2.
pub type PCROP2_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PCROP2EN` reader - PCROP2 area enable
pub type PCROP2EN_R = crate::BitReader;
///Field `PCROP2EN` writer - PCROP2 area enable
pub type PCROP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDP2_PEND` reader - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
pub type HDP2_PEND_R = crate::FieldReader;
///Field `HDP2_PEND` writer - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
pub type HDP2_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HDP2EN` reader - Hide protection second area enable
pub type HDP2EN_R = crate::BitReader;
///Field `HDP2EN` writer - Hide protection second area enable
pub type HDP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2.
    #[inline(always)]
    pub fn pcrop2_pstrt(&self) -> PCROP2_PSTRT_R {
        PCROP2_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 15 - PCROP2 area enable
    #[inline(always)]
    pub fn pcrop2en(&self) -> PCROP2EN_R {
        PCROP2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
    #[inline(always)]
    pub fn hdp2_pend(&self) -> HDP2_PEND_R {
        HDP2_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - Hide protection second area enable
    #[inline(always)]
    pub fn hdp2en(&self) -> HDP2EN_R {
        HDP2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM2R2")
            .field("pcrop2_pstrt", &self.pcrop2_pstrt())
            .field("pcrop2en", &self.pcrop2en())
            .field("hdp2_pend", &self.hdp2_pend())
            .field("hdp2en", &self.hdp2en())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Start page of PCROP2 area PRCROP2_PSTRT contains the first page of the PCROP area in bank 2.
    #[inline(always)]
    pub fn pcrop2_pstrt(&mut self) -> PCROP2_PSTRT_W<SECWM2R2rs> {
        PCROP2_PSTRT_W::new(self, 0)
    }
    ///Bit 15 - PCROP2 area enable
    #[inline(always)]
    pub fn pcrop2en(&mut self) -> PCROP2EN_W<SECWM2R2rs> {
        PCROP2EN_W::new(self, 15)
    }
    ///Bits 16:23 - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
    #[inline(always)]
    pub fn hdp2_pend(&mut self) -> HDP2_PEND_W<SECWM2R2rs> {
        HDP2_PEND_W::new(self, 16)
    }
    ///Bit 31 - Hide protection second area enable
    #[inline(always)]
    pub fn hdp2en(&mut self) -> HDP2EN_W<SECWM2R2rs> {
        HDP2EN_W::new(self, 31)
    }
}
/**FLASH secure watermark2 register 2

You can [`read`](crate::Reg::read) this register and get [`secwm2r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FLASH:SECWM2R2)*/
pub struct SECWM2R2rs;
impl crate::RegisterSpec for SECWM2R2rs {
    type Ux = u32;
}
///`read()` method returns [`secwm2r2::R`](R) reader structure
impl crate::Readable for SECWM2R2rs {}
///`write(|w| ..)` method takes [`secwm2r2::W`](W) writer structure
impl crate::Writable for SECWM2R2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWM2R2 to value 0x0f00_0f00
impl crate::Resettable for SECWM2R2rs {
    const RESET_VALUE: u32 = 0x0f00_0f00;
}
