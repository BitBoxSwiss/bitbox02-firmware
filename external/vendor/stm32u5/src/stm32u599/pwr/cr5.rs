///Register `CR5` reader
pub type R = crate::R<CR5rs>;
///Register `CR5` writer
pub type W = crate::W<CR5rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**

You can [`read`](crate::Reg::read) this register and get [`cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#PWR:CR5)*/
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
///`read()` method returns [`cr5::R`](R) reader structure
impl crate::Readable for CR5rs {}
///`write(|w| ..)` method takes [`cr5::W`](W) writer structure
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR5 to value 0
impl crate::Resettable for CR5rs {}
