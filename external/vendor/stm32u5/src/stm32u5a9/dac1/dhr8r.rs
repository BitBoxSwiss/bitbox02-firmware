///Register `DHR8R%s` reader
pub type R = crate::R<DHR8Rrs>;
///Register `DHR8R%s` writer
pub type W = crate::W<DHR8Rrs>;
///Field `DACCDHR` reader - DAC channel1 8-bit right-aligned data
pub type DACCDHR_R = crate::FieldReader;
///Field `DACCDHR` writer - DAC channel1 8-bit right-aligned data
pub type DACCDHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `DACC1DHRB` reader - DAC channel1 8-bit right-aligned Sdata
pub type DACC1DHRB_R = crate::FieldReader;
///Field `DACC1DHRB` writer - DAC channel1 8-bit right-aligned Sdata
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn daccdhr(&self) -> DACCDHR_R {
        DACCDHR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DAC channel1 8-bit right-aligned Sdata
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR8R")
            .field("daccdhr", &self.daccdhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn daccdhr(&mut self) -> DACCDHR_W<DHR8Rrs> {
        DACCDHR_W::new(self, 0)
    }
    ///Bits 8:15 - DAC channel1 8-bit right-aligned Sdata
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR8Rrs> {
        DACC1DHRB_W::new(self, 8)
    }
}
/**channel%s 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DAC1:DHR8R[1])*/
pub struct DHR8Rrs;
impl crate::RegisterSpec for DHR8Rrs {
    type Ux = u32;
}
///`read()` method returns [`dhr8r::R`](R) reader structure
impl crate::Readable for DHR8Rrs {}
///`write(|w| ..)` method takes [`dhr8r::W`](W) writer structure
impl crate::Writable for DHR8Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR8R%s to value 0
impl crate::Resettable for DHR8Rrs {}
