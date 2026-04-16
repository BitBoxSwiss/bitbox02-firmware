///Register `WRP1BR` reader
pub type R = crate::R<WRP1BRrs>;
///Register `WRP1BR` writer
pub type W = crate::W<WRP1BRrs>;
///Field `WRP1B_PSTRT` reader - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
pub type WRP1B_PSTRT_R = crate::FieldReader;
///Field `WRP1B_PSTRT` writer - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
pub type WRP1B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `WRP1B_PEND` reader - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
pub type WRP1B_PEND_R = crate::FieldReader;
///Field `WRP1B_PEND` writer - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
pub type WRP1B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `UNLOCK` reader - Bank 1 WPR second area B unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - Bank 1 WPR second area B unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
    #[inline(always)]
    pub fn wrp1b_pstrt(&self) -> WRP1B_PSTRT_R {
        WRP1B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
    #[inline(always)]
    pub fn wrp1b_pend(&self) -> WRP1B_PEND_R {
        WRP1B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - Bank 1 WPR second area B unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1BR")
            .field("wrp1b_pstrt", &self.wrp1b_pstrt())
            .field("wrp1b_pend", &self.wrp1b_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1.
    #[inline(always)]
    pub fn wrp1b_pstrt(&mut self) -> WRP1B_PSTRT_W<WRP1BRrs> {
        WRP1B_PSTRT_W::new(self, 0)
    }
    ///Bits 16:22 - Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1.
    #[inline(always)]
    pub fn wrp1b_pend(&mut self) -> WRP1B_PEND_W<WRP1BRrs> {
        WRP1B_PEND_W::new(self, 16)
    }
    ///Bit 31 - Bank 1 WPR second area B unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<WRP1BRrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WRP1 area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp1br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FLASH:WRP1BR)*/
pub struct WRP1BRrs;
impl crate::RegisterSpec for WRP1BRrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1br::R`](R) reader structure
impl crate::Readable for WRP1BRrs {}
///`write(|w| ..)` method takes [`wrp1br::W`](W) writer structure
impl crate::Writable for WRP1BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP1BR to value 0x0f00_ff00
impl crate::Resettable for WRP1BRrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
