///Register `CSR19` reader
pub type R = crate::R<CSR19rs>;
///Register `CSR19` writer
pub type W = crate::W<CSR19rs>;
///Field `CS19` reader - CS19
pub type CS19_R = crate::FieldReader<u32>;
///Field `CS19` writer - CS19
pub type CS19_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS19
    #[inline(always)]
    pub fn cs19(&self) -> CS19_R {
        CS19_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR19").field("cs19", &self.cs19()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS19
    #[inline(always)]
    pub fn cs19(&mut self) -> CS19_W<CSR19rs> {
        CS19_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#HASH:CSR19)*/
pub struct CSR19rs;
impl crate::RegisterSpec for CSR19rs {
    type Ux = u32;
}
///`read()` method returns [`csr19::R`](R) reader structure
impl crate::Readable for CSR19rs {}
///`write(|w| ..)` method takes [`csr19::W`](W) writer structure
impl crate::Writable for CSR19rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR19 to value 0
impl crate::Resettable for CSR19rs {}
