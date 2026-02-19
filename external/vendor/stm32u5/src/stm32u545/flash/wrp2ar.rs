///Register `WRP2AR` reader
pub type R = crate::R<WRP2ARrs>;
///Register `WRP2AR` writer
pub type W = crate::W<WRP2ARrs>;
///Field `WRP2A_PSTRT` reader - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2.
pub type WRP2A_PSTRT_R = crate::FieldReader;
///Field `WRP2A_PSTRT` writer - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2.
pub type WRP2A_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRP2A_PEND` reader - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2.
pub type WRP2A_PEND_R = crate::FieldReader;
///Field `WRP2A_PEND` writer - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2.
pub type WRP2A_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UNLOCK` reader - Bank 2 WPR first area A unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - Bank 2 WPR first area A unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2.
    #[inline(always)]
    pub fn wrp2a_pstrt(&self) -> WRP2A_PSTRT_R {
        WRP2A_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2.
    #[inline(always)]
    pub fn wrp2a_pend(&self) -> WRP2A_PEND_R {
        WRP2A_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - Bank 2 WPR first area A unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP2AR")
            .field("wrp2a_pstrt", &self.wrp2a_pstrt())
            .field("wrp2a_pend", &self.wrp2a_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2.
    #[inline(always)]
    pub fn wrp2a_pstrt(&mut self) -> WRP2A_PSTRT_W<WRP2ARrs> {
        WRP2A_PSTRT_W::new(self, 0)
    }
    ///Bits 16:23 - Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2.
    #[inline(always)]
    pub fn wrp2a_pend(&mut self) -> WRP2A_PEND_W<WRP2ARrs> {
        WRP2A_PEND_W::new(self, 16)
    }
    ///Bit 31 - Bank 2 WPR first area A unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<WRP2ARrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WPR2 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FLASH:WRP2AR)*/
pub struct WRP2ARrs;
impl crate::RegisterSpec for WRP2ARrs {
    type Ux = u32;
}
///`read()` method returns [`wrp2ar::R`](R) reader structure
impl crate::Readable for WRP2ARrs {}
///`write(|w| ..)` method takes [`wrp2ar::W`](W) writer structure
impl crate::Writable for WRP2ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP2AR to value 0x0f00_ff00
impl crate::Resettable for WRP2ARrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
