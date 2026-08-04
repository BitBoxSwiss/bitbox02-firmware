///Register `M5ECCKEYR` reader
pub type R = crate::R<M5ECCKEYRrs>;
///Register `M5ECCKEYR` writer
pub type W = crate::W<M5ECCKEYRrs>;
///Field `ECCKEY` reader - ECCKEY
pub type ECCKEY_R = crate::FieldReader;
///Field `ECCKEY` writer - ECCKEY
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - ECCKEY
    #[inline(always)]
    pub fn ecckey(&self) -> ECCKEY_R {
        ECCKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5ECCKEYR")
            .field("ecckey", &self.ecckey())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ECCKEY
    #[inline(always)]
    pub fn ecckey(&mut self) -> ECCKEY_W<M5ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
/**RAMCFG RAM x interrupt clear register x

You can [`read`](crate::Reg::read) this register and get [`m5ecckeyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ecckeyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RAMCFG:M5ECCKEYR)*/
pub struct M5ECCKEYRrs;
impl crate::RegisterSpec for M5ECCKEYRrs {
    type Ux = u32;
}
///`read()` method returns [`m5ecckeyr::R`](R) reader structure
impl crate::Readable for M5ECCKEYRrs {}
///`write(|w| ..)` method takes [`m5ecckeyr::W`](W) writer structure
impl crate::Writable for M5ECCKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5ECCKEYR to value 0
impl crate::Resettable for M5ECCKEYRrs {}
