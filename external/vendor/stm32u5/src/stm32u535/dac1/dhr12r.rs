///Register `DHR12R%s` reader
pub type R = crate::R<DHR12Rrs>;
///Register `DHR12R%s` writer
pub type W = crate::W<DHR12Rrs>;
///Field `DACCDHR` reader - DAC channel1 12-bit right-aligned data
pub type DACCDHR_R = crate::FieldReader<u16>;
///Field `DACCDHR` writer - DAC channel1 12-bit right-aligned data
pub type DACCDHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `DACC1DHRB` reader - DAC channel1 12-bit right-aligned data B
pub type DACC1DHRB_R = crate::FieldReader<u16>;
///Field `DACC1DHRB` writer - DAC channel1 12-bit right-aligned data B
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data
    #[inline(always)]
    pub fn daccdhr(&self) -> DACCDHR_R {
        DACCDHR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - DAC channel1 12-bit right-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12R")
            .field("daccdhr", &self.daccdhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - DAC channel1 12-bit right-aligned data
    #[inline(always)]
    pub fn daccdhr(&mut self) -> DACCDHR_W<DHR12Rrs> {
        DACCDHR_W::new(self, 0)
    }
    ///Bits 16:27 - DAC channel1 12-bit right-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR12Rrs> {
        DACC1DHRB_W::new(self, 16)
    }
}
/**channel%s 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DAC1:DHR12R[1])*/
pub struct DHR12Rrs;
impl crate::RegisterSpec for DHR12Rrs {
    type Ux = u32;
}
///`read()` method returns [`dhr12r::R`](R) reader structure
impl crate::Readable for DHR12Rrs {}
///`write(|w| ..)` method takes [`dhr12r::W`](W) writer structure
impl crate::Writable for DHR12Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12R%s to value 0
impl crate::Resettable for DHR12Rrs {}
