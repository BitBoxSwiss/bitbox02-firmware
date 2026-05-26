///Register `SECWM1R2` reader
pub type R = crate::R<SECWM1R2rs>;
///Register `SECWM1R2` writer
pub type W = crate::W<SECWM1R2rs>;
///Field `HDP1_PEND` reader - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
pub type HDP1_PEND_R = crate::FieldReader;
///Field `HDP1_PEND` writer - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
pub type HDP1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HDP1EN` reader - Hide protection first area enable
pub type HDP1EN_R = crate::BitReader;
///Field `HDP1EN` writer - Hide protection first area enable
pub type HDP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:22 - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - Hide protection first area enable
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM1R2")
            .field("hdp1_pend", &self.hdp1_pend())
            .field("hdp1en", &self.hdp1en())
            .finish()
    }
}
impl W {
    ///Bits 16:22 - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
    #[inline(always)]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<SECWM1R2rs> {
        HDP1_PEND_W::new(self, 16)
    }
    ///Bit 31 - Hide protection first area enable
    #[inline(always)]
    pub fn hdp1en(&mut self) -> HDP1EN_W<SECWM1R2rs> {
        HDP1EN_W::new(self, 31)
    }
}
/**FLASH secure watermark1 register 2

You can [`read`](crate::Reg::read) this register and get [`secwm1r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FLASH:SECWM1R2)*/
pub struct SECWM1R2rs;
impl crate::RegisterSpec for SECWM1R2rs {
    type Ux = u32;
}
///`read()` method returns [`secwm1r2::R`](R) reader structure
impl crate::Readable for SECWM1R2rs {}
///`write(|w| ..)` method takes [`secwm1r2::W`](W) writer structure
impl crate::Writable for SECWM1R2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWM1R2 to value 0x0f00_ffff
impl crate::Resettable for SECWM1R2rs {
    const RESET_VALUE: u32 = 0x0f00_ffff;
}
