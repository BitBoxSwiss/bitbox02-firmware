///Register `R2NONCER0` reader
pub type R = crate::R<R2NONCER0rs>;
///Register `R2NONCER0` writer
pub type W = crate::W<R2NONCER0rs>;
///Field `REG2_NONCE` reader - REG2_NONCE
pub type REG2_NONCE_R = crate::FieldReader<u32>;
///Field `REG2_NONCE` writer - REG2_NONCE
pub type REG2_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - REG2_NONCE
    #[inline(always)]
    pub fn reg2_nonce(&self) -> REG2_NONCE_R {
        REG2_NONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R2NONCER0")
            .field("reg2_nonce", &self.reg2_nonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - REG2_NONCE
    #[inline(always)]
    pub fn reg2_nonce(&mut self) -> REG2_NONCE_W<R2NONCER0rs> {
        REG2_NONCE_W::new(self, 0)
    }
}
/**OTFDEC region x nonce register 0

You can [`read`](crate::Reg::read) this register and get [`r2noncer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2noncer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTFDEC1:R2NONCER0)*/
pub struct R2NONCER0rs;
impl crate::RegisterSpec for R2NONCER0rs {
    type Ux = u32;
}
///`read()` method returns [`r2noncer0::R`](R) reader structure
impl crate::Readable for R2NONCER0rs {}
///`write(|w| ..)` method takes [`r2noncer0::W`](W) writer structure
impl crate::Writable for R2NONCER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R2NONCER0 to value 0
impl crate::Resettable for R2NONCER0rs {}
