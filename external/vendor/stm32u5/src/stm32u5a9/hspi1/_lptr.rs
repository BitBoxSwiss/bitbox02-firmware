///Register `_LPTR` reader
pub type R = crate::R<_LPTRrs>;
///Register `_LPTR` writer
pub type W = crate::W<_LPTRrs>;
///Field `TIMEOUT` reader - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
pub type TIMEOUT_R = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_LPTR")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - 15: 0\]: Timeout period After each access in Memory-mapped mode, the HSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the HSPI waits after the clock becomes inactive and until it raises the nCS, putting the external device in a lower-consumption state.
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<_LPTRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
/**HSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`_lptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_lptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_LPTR)*/
pub struct _LPTRrs;
impl crate::RegisterSpec for _LPTRrs {
    type Ux = u32;
}
///`read()` method returns [`_lptr::R`](R) reader structure
impl crate::Readable for _LPTRrs {}
///`write(|w| ..)` method takes [`_lptr::W`](W) writer structure
impl crate::Writable for _LPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _LPTR to value 0
impl crate::Resettable for _LPTRrs {}
