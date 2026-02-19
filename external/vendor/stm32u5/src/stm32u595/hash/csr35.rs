///Register `CSR35` reader
pub type R = crate::R<CSR35rs>;
///Register `CSR35` writer
pub type W = crate::W<CSR35rs>;
///Field `CS35` reader - CS35
pub type CS35_R = crate::FieldReader<u32>;
///Field `CS35` writer - CS35
pub type CS35_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CS35
    #[inline(always)]
    pub fn cs35(&self) -> CS35_R {
        CS35_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR35").field("cs35", &self.cs35()).finish()
    }
}
impl W {
    ///Bits 0:31 - CS35
    #[inline(always)]
    pub fn cs35(&mut self) -> CS35_W<CSR35rs> {
        CS35_W::new(self, 0)
    }
}
/**context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:CSR35)*/
pub struct CSR35rs;
impl crate::RegisterSpec for CSR35rs {
    type Ux = u32;
}
///`read()` method returns [`csr35::R`](R) reader structure
impl crate::Readable for CSR35rs {}
///`write(|w| ..)` method takes [`csr35::W`](W) writer structure
impl crate::Writable for CSR35rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR35 to value 0
impl crate::Resettable for CSR35rs {}
