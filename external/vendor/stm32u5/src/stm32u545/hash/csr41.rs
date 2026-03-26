///Register `CSR41` reader
pub type R = crate::R<CSR41rs>;
///Register `CSR41` writer
pub type W = crate::W<CSR41rs>;
///Field `CS41` reader - CS41
pub type CS41_R = crate::FieldReader<u32>;
///Field `CS41` writer - CS41
pub type CS41_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS41
    #[inline(always)]
    pub fn cs41(&self) -> CS41_R {
        CS41_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR41").field("cs41", &self.cs41()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS41
    #[inline(always)]
    pub fn cs41(&mut self) -> CS41_W<CSR41rs> {
        CS41_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR41)*/
pub struct CSR41rs;
impl crate::RegisterSpec for CSR41rs {
    type Ux = u32;
}
///`read()` method returns [`csr41::R`](R) reader structure
impl crate::Readable for CSR41rs {}
///`write(|w| ..)` method takes [`csr41::W`](W) writer structure
impl crate::Writable for CSR41rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR41 to value 0
impl crate::Resettable for CSR41rs {}
