///Register `_HLCR` reader
pub type R = crate::R<_HLCRrs>;
///Register `_HLCR` writer
pub type W = crate::W<_HLCRrs>;
///Field `LM` reader - Latency mode This bit selects the Latency mode.
pub type LM_R = crate::BitReader;
///Field `LM` writer - Latency mode This bit selects the Latency mode.
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WZL` reader - Write zero latency This bit enables zero latency on write operations.
pub type WZL_R = crate::BitReader;
///Field `WZL` writer - Write zero latency This bit enables zero latency on write operations.
pub type WZL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TACC` reader - 7: 0\]: Access time Device access time expressed in number of communication clock cycles
pub type TACC_R = crate::FieldReader;
///Field `TACC` writer - 7: 0\]: Access time Device access time expressed in number of communication clock cycles
pub type TACC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TRWR` reader - Read write recovery time Device read write recovery time expressed in number of communication clock cycles
pub type TRWR_R = crate::FieldReader;
///Field `TRWR` writer - Read write recovery time Device read write recovery time expressed in number of communication clock cycles
pub type TRWR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Latency mode This bit selects the Latency mode.
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write zero latency This bit enables zero latency on write operations.
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:15 - 7: 0\]: Access time Device access time expressed in number of communication clock cycles
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Read write recovery time Device read write recovery time expressed in number of communication clock cycles
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_HLCR")
            .field("lm", &self.lm())
            .field("wzl", &self.wzl())
            .field("tacc", &self.tacc())
            .field("trwr", &self.trwr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Latency mode This bit selects the Latency mode.
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<_HLCRrs> {
        LM_W::new(self, 0)
    }
    ///Bit 1 - Write zero latency This bit enables zero latency on write operations.
    #[inline(always)]
    pub fn wzl(&mut self) -> WZL_W<_HLCRrs> {
        WZL_W::new(self, 1)
    }
    ///Bits 8:15 - 7: 0\]: Access time Device access time expressed in number of communication clock cycles
    #[inline(always)]
    pub fn tacc(&mut self) -> TACC_W<_HLCRrs> {
        TACC_W::new(self, 8)
    }
    ///Bits 16:23 - Read write recovery time Device read write recovery time expressed in number of communication clock cycles
    #[inline(always)]
    pub fn trwr(&mut self) -> TRWR_W<_HLCRrs> {
        TRWR_W::new(self, 16)
    }
}
/**HSPI HyperBus latency configuration register

You can [`read`](crate::Reg::read) this register and get [`_hlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_hlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_HLCR)*/
pub struct _HLCRrs;
impl crate::RegisterSpec for _HLCRrs {
    type Ux = u32;
}
///`read()` method returns [`_hlcr::R`](R) reader structure
impl crate::Readable for _HLCRrs {}
///`write(|w| ..)` method takes [`_hlcr::W`](W) writer structure
impl crate::Writable for _HLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _HLCR to value 0
impl crate::Resettable for _HLCRrs {}
