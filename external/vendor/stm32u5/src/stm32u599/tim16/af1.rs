///Register `AF1` reader
pub type R = crate::R<AF1rs>;
///Register `AF1` writer
pub type W = crate::W<AF1rs>;
///Field `BKINE` reader - TIMx_BKIN input enable
pub type BKINE_R = crate::BitReader;
///Field `BKINE` writer - TIMx_BKIN input enable
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP1E` reader - tim_brk_cmp1 enable
pub type BKCMP1E_R = crate::BitReader;
///Field `BKCMP1E` writer - tim_brk_cmp1 enable
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP2E` reader - tim_brk_cmp2 enable
pub type BKCMP2E_R = crate::BitReader;
///Field `BKCMP2E` writer - tim_brk_cmp2 enable
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP3E` reader - tim_brk_cmp3 enable
pub type BKCMP3E_R = crate::BitReader;
///Field `BKCMP3E` writer - tim_brk_cmp3 enable
pub type BKCMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP4E` reader - tim_brk_cmp4 enable
pub type BKCMP4E_R = crate::BitReader;
///Field `BKCMP4E` writer - tim_brk_cmp4 enable
pub type BKCMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP5E` reader - tim_brk_cmp5 enable
pub type BKCMP5E_R = crate::BitReader;
///Field `BKCMP5E` writer - tim_brk_cmp5 enable
pub type BKCMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP6E` reader - tim_brk_cmp6 enable
pub type BKCMP6E_R = crate::BitReader;
///Field `BKCMP6E` writer - tim_brk_cmp6 enable
pub type BKCMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP7E` reader - tim_brk_cmp7 enable
pub type BKCMP7E_R = crate::BitReader;
///Field `BKCMP7E` writer - tim_brk_cmp7 enable
pub type BKCMP7E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKINP` reader - TIMx_BKIN input polarity
pub type BKINP_R = crate::BitReader;
///Field `BKINP` writer - TIMx_BKIN input polarity
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP1P` reader - tim_brk_cmp1 input polarity
pub type BKCMP1P_R = crate::BitReader;
///Field `BKCMP1P` writer - tim_brk_cmp1 input polarity
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP2P` reader - tim_brk_cmp2 input polarity
pub type BKCMP2P_R = crate::BitReader;
///Field `BKCMP2P` writer - tim_brk_cmp2 input polarity
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP3P` reader - tim_brk_cmp3 input polarity
pub type BKCMP3P_R = crate::BitReader;
///Field `BKCMP3P` writer - tim_brk_cmp3 input polarity
pub type BKCMP3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKCMP4P` reader - tim_brk_cmp4 input polarity
pub type BKCMP4P_R = crate::BitReader;
///Field `BKCMP4P` writer - tim_brk_cmp4 input polarity
pub type BKCMP4P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIMx_BKIN input enable
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - tim_brk_cmp1 enable
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tim_brk_cmp2 enable
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - tim_brk_cmp3 enable
    #[inline(always)]
    pub fn bkcmp3e(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - tim_brk_cmp4 enable
    #[inline(always)]
    pub fn bkcmp4e(&self) -> BKCMP4E_R {
        BKCMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - tim_brk_cmp5 enable
    #[inline(always)]
    pub fn bkcmp5e(&self) -> BKCMP5E_R {
        BKCMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - tim_brk_cmp6 enable
    #[inline(always)]
    pub fn bkcmp6e(&self) -> BKCMP6E_R {
        BKCMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - tim_brk_cmp7 enable
    #[inline(always)]
    pub fn bkcmp7e(&self) -> BKCMP7E_R {
        BKCMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - TIMx_BKIN input polarity
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - tim_brk_cmp1 input polarity
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - tim_brk_cmp2 input polarity
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - tim_brk_cmp3 input polarity
    #[inline(always)]
    pub fn bkcmp3p(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - tim_brk_cmp4 input polarity
    #[inline(always)]
    pub fn bkcmp4p(&self) -> BKCMP4P_R {
        BKCMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF1")
            .field("bkcmp4p", &self.bkcmp4p())
            .field("bkcmp3p", &self.bkcmp3p())
            .field("bkcmp2p", &self.bkcmp2p())
            .field("bkcmp1p", &self.bkcmp1p())
            .field("bkinp", &self.bkinp())
            .field("bkcmp7e", &self.bkcmp7e())
            .field("bkcmp6e", &self.bkcmp6e())
            .field("bkcmp5e", &self.bkcmp5e())
            .field("bkcmp4e", &self.bkcmp4e())
            .field("bkcmp3e", &self.bkcmp3e())
            .field("bkcmp2e", &self.bkcmp2e())
            .field("bkcmp1e", &self.bkcmp1e())
            .field("bkine", &self.bkine())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIMx_BKIN input enable
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W<AF1rs> {
        BKINE_W::new(self, 0)
    }
    ///Bit 1 - tim_brk_cmp1 enable
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    ///Bit 2 - tim_brk_cmp2 enable
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    ///Bit 3 - tim_brk_cmp3 enable
    #[inline(always)]
    pub fn bkcmp3e(&mut self) -> BKCMP3E_W<AF1rs> {
        BKCMP3E_W::new(self, 3)
    }
    ///Bit 4 - tim_brk_cmp4 enable
    #[inline(always)]
    pub fn bkcmp4e(&mut self) -> BKCMP4E_W<AF1rs> {
        BKCMP4E_W::new(self, 4)
    }
    ///Bit 5 - tim_brk_cmp5 enable
    #[inline(always)]
    pub fn bkcmp5e(&mut self) -> BKCMP5E_W<AF1rs> {
        BKCMP5E_W::new(self, 5)
    }
    ///Bit 6 - tim_brk_cmp6 enable
    #[inline(always)]
    pub fn bkcmp6e(&mut self) -> BKCMP6E_W<AF1rs> {
        BKCMP6E_W::new(self, 6)
    }
    ///Bit 7 - tim_brk_cmp7 enable
    #[inline(always)]
    pub fn bkcmp7e(&mut self) -> BKCMP7E_W<AF1rs> {
        BKCMP7E_W::new(self, 7)
    }
    ///Bit 9 - TIMx_BKIN input polarity
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W<AF1rs> {
        BKINP_W::new(self, 9)
    }
    ///Bit 10 - tim_brk_cmp1 input polarity
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    ///Bit 11 - tim_brk_cmp2 input polarity
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
    ///Bit 12 - tim_brk_cmp3 input polarity
    #[inline(always)]
    pub fn bkcmp3p(&mut self) -> BKCMP3P_W<AF1rs> {
        BKCMP3P_W::new(self, 12)
    }
    ///Bit 13 - tim_brk_cmp4 input polarity
    #[inline(always)]
    pub fn bkcmp4p(&mut self) -> BKCMP4P_W<AF1rs> {
        BKCMP4P_W::new(self, 13)
    }
}
/**alternate function register 1

You can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TIM16:AF1)*/
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
///`read()` method returns [`af1::R`](R) reader structure
impl crate::Readable for AF1rs {}
///`write(|w| ..)` method takes [`af1::W`](W) writer structure
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF1 to value 0x01
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
