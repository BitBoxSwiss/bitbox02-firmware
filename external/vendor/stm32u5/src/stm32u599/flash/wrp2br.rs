///Register `WRP2BR` reader
pub type R = crate::R<WRP2BRrs>;
///Register `WRP2BR` writer
pub type W = crate::W<WRP2BRrs>;
///Field `WRP2B_PSTRT` reader - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2.
pub type WRP2B_PSTRT_R = crate::FieldReader;
///Field `WRP2B_PSTRT` writer - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2.
pub type WRP2B_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRP2B_PEND` reader - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2.
pub type WRP2B_PEND_R = crate::FieldReader;
///Field `WRP2B_PEND` writer - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2.
pub type WRP2B_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UNLOCK` reader - Bank 2 WPR second area B unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - Bank 2 WPR second area B unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2.
    #[inline(always)]
    pub fn wrp2b_pstrt(&self) -> WRP2B_PSTRT_R {
        WRP2B_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2.
    #[inline(always)]
    pub fn wrp2b_pend(&self) -> WRP2B_PEND_R {
        WRP2B_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - Bank 2 WPR second area B unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2BR")
            .field("wrp2b_pstrt", &self.wrp2b_pstrt())
            .field("wrp2b_pend", &self.wrp2b_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2.
    #[inline(always)]
    pub fn wrp2b_pstrt(&mut self) -> WRP2B_PSTRT_W<WRP2BRrs> {
        WRP2B_PSTRT_W::new(self, 0)
    }
    ///Bits 16:23 - Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2.
    #[inline(always)]
    pub fn wrp2b_pend(&mut self) -> WRP2B_PEND_W<WRP2BRrs> {
        WRP2B_PEND_W::new(self, 16)
    }
    ///Bit 31 - Bank 2 WPR second area B unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<WRP2BRrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WPR2 area B address register

You can [`read`](crate::Reg::read) this register and get [`wrp2br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:WRP2BR)*/
pub struct WRP2BRrs;
impl crate::RegisterSpec for WRP2BRrs {
    type Ux = u32;
}
///`read()` method returns [`wrp2br::R`](R) reader structure
impl crate::Readable for WRP2BRrs {}
///`write(|w| ..)` method takes [`wrp2br::W`](W) writer structure
impl crate::Writable for WRP2BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP2BR to value 0x0f00_ff00
impl crate::Resettable for WRP2BRrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
