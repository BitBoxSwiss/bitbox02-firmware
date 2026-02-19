///Register `R1NONCER0` reader
pub type R = crate::R<R1NONCER0rs>;
///Register `R1NONCER0` writer
pub type W = crate::W<R1NONCER0rs>;
///Field `REG1_NONCE` reader - REG1_NONCE
pub type REG1_NONCE_R = crate::FieldReader<u32>;
///Field `REG1_NONCE` writer - REG1_NONCE
pub type REG1_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - REG1_NONCE
    #[inline(always)]
    pub fn reg1_nonce(&self) -> REG1_NONCE_R {
        REG1_NONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R1NONCER0")
            .field("reg1_nonce", &self.reg1_nonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - REG1_NONCE
    #[inline(always)]
    pub fn reg1_nonce(&mut self) -> REG1_NONCE_W<R1NONCER0rs> {
        REG1_NONCE_W::new(self, 0)
    }
}
/**OTFDEC region x nonce register 0

You can [`read`](crate::Reg::read) this register and get [`r1noncer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r1noncer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#OTFDEC1:R1NONCER0)*/
pub struct R1NONCER0rs;
impl crate::RegisterSpec for R1NONCER0rs {
    type Ux = u32;
}
///`read()` method returns [`r1noncer0::R`](R) reader structure
impl crate::Readable for R1NONCER0rs {}
///`write(|w| ..)` method takes [`r1noncer0::W`](W) writer structure
impl crate::Writable for R1NONCER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R1NONCER0 to value 0
impl crate::Resettable for R1NONCER0rs {}
