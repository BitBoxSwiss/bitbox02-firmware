///Register `WRP1AR` reader
pub type R = crate::R<WRP1ARrs>;
///Register `WRP1AR` writer
pub type W = crate::W<WRP1ARrs>;
///Field `WRP1A_PSTRT` reader - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
pub type WRP1A_PSTRT_R = crate::FieldReader;
///Field `WRP1A_PSTRT` writer - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
pub type WRP1A_PSTRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WRP1A_PEND` reader - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
pub type WRP1A_PEND_R = crate::FieldReader;
///Field `WRP1A_PEND` writer - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
pub type WRP1A_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UNLOCK` reader - Bank 1 WPR first area A unlock
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - Bank 1 WPR first area A unlock
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
    #[inline(always)]
    pub fn wrp1a_pstrt(&self) -> WRP1A_PSTRT_R {
        WRP1A_PSTRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
    #[inline(always)]
    pub fn wrp1a_pend(&self) -> WRP1A_PEND_R {
        WRP1A_PEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - Bank 1 WPR first area A unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRP1AR")
            .field("wrp1a_pstrt", &self.wrp1a_pstrt())
            .field("wrp1a_pend", &self.wrp1a_pend())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
    #[inline(always)]
    pub fn wrp1a_pstrt(&mut self) -> WRP1A_PSTRT_W<WRP1ARrs> {
        WRP1A_PSTRT_W::new(self, 0)
    }
    ///Bits 16:23 - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
    #[inline(always)]
    pub fn wrp1a_pend(&mut self) -> WRP1A_PEND_W<WRP1ARrs> {
        WRP1A_PEND_W::new(self, 16)
    }
    ///Bit 31 - Bank 1 WPR first area A unlock
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<WRP1ARrs> {
        UNLOCK_W::new(self, 31)
    }
}
/**FLASH WRP1 area A address register

You can [`read`](crate::Reg::read) this register and get [`wrp1ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp1ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:WRP1AR)*/
pub struct WRP1ARrs;
impl crate::RegisterSpec for WRP1ARrs {
    type Ux = u32;
}
///`read()` method returns [`wrp1ar::R`](R) reader structure
impl crate::Readable for WRP1ARrs {}
///`write(|w| ..)` method takes [`wrp1ar::W`](W) writer structure
impl crate::Writable for WRP1ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRP1AR to value 0x0f00_ff00
impl crate::Resettable for WRP1ARrs {
    const RESET_VALUE: u32 = 0x0f00_ff00;
}
