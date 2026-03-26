///Register `SECWM2R1` reader
pub type R = crate::R<SECWM2R1rs>;
///Register `SECWM2R1` writer
pub type W = crate::W<SECWM2R1rs>;
///Field `SECWM2_PSTRT` reader - Start page of second secure area This field contains the first page of the secure area in bank 2.
pub type SECWM2_PSTRT_R = crate::FieldReader;
///Field `SECWM2_PSTRT` writer - Start page of second secure area This field contains the first page of the secure area in bank 2.
pub type SECWM2_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SECWM2_PEND` reader - End page of second secure area This field contains the last page of the secure area in bank 2.
pub type SECWM2_PEND_R = crate::FieldReader;
///Field `SECWM2_PEND` writer - End page of second secure area This field contains the last page of the secure area in bank 2.
pub type SECWM2_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Start page of second secure area This field contains the first page of the secure area in bank 2.
    #[inline(always)]
    pub fn secwm2_pstrt(&self) -> SECWM2_PSTRT_R {
        SECWM2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - End page of second secure area This field contains the last page of the secure area in bank 2.
    #[inline(always)]
    pub fn secwm2_pend(&self) -> SECWM2_PEND_R {
        SECWM2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM2R1")
            .field("secwm2_pstrt", &self.secwm2_pstrt())
            .field("secwm2_pend", &self.secwm2_pend())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Start page of second secure area This field contains the first page of the secure area in bank 2.
    #[inline(always)]
    pub fn secwm2_pstrt(&mut self) -> SECWM2_PSTRT_W<SECWM2R1rs> {
        SECWM2_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - End page of second secure area This field contains the last page of the secure area in bank 2.
    #[inline(always)]
    pub fn secwm2_pend(&mut self) -> SECWM2_PEND_W<SECWM2R1rs> {
        SECWM2_PEND_W::new(self, 16)
    }
}
/**FLASH secure watermark2 register 1

You can [`read`](crate::Reg::read) this register and get [`secwm2r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FLASH:SECWM2R1)*/
pub struct SECWM2R1rs;
impl crate::RegisterSpec for SECWM2R1rs {
    type Ux = u32;
}
///`read()` method returns [`secwm2r1::R`](R) reader structure
impl crate::Readable for SECWM2R1rs {}
///`write(|w| ..)` method takes [`secwm2r1::W`](W) writer structure
impl crate::Writable for SECWM2R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWM2R1 to value 0xff00_ff00
impl crate::Resettable for SECWM2R1rs {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
