///Register `R3NONCER0` reader
pub type R = crate::R<R3NONCER0rs>;
///Register `R3NONCER0` writer
pub type W = crate::W<R3NONCER0rs>;
///Field `REGx_NONCE` reader - REGx_NONCE
pub type REGX_NONCE_R = crate::FieldReader<u32>;
///Field `REGx_NONCE` writer - REGx_NONCE
pub type REGX_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - REGx_NONCE
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R3NONCER0")
            .field("regx_nonce", &self.regx_nonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - REGx_NONCE
    #[inline(always)]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<R3NONCER0rs> {
        REGX_NONCE_W::new(self, 0)
    }
}
/**OTFDEC region x nonce register 0

You can [`read`](crate::Reg::read) this register and get [`r3noncer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3noncer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OTFDEC1:R3NONCER0)*/
pub struct R3NONCER0rs;
impl crate::RegisterSpec for R3NONCER0rs {
    type Ux = u32;
}
///`read()` method returns [`r3noncer0::R`](R) reader structure
impl crate::Readable for R3NONCER0rs {}
///`write(|w| ..)` method takes [`r3noncer0::W`](W) writer structure
impl crate::Writable for R3NONCER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3NONCER0 to value 0
impl crate::Resettable for R3NONCER0rs {}
