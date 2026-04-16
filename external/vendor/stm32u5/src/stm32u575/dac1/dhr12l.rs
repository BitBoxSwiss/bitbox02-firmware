///Register `DHR12L%s` reader
pub type R = crate::R<DHR12Lrs>;
///Register `DHR12L%s` writer
pub type W = crate::W<DHR12Lrs>;
///Field `DACCDHR` reader - DAC channel1 12-bit left-aligned data
pub type DACCDHR_R = crate::FieldReader<u16>;
///Field `DACCDHR` writer - DAC channel1 12-bit left-aligned data
pub type DACCDHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `DACC1DHRB` reader - DAC channel1 12-bit left-aligned data B
pub type DACC1DHRB_R = crate::FieldReader<u16>;
///Field `DACC1DHRB` writer - DAC channel1 12-bit left-aligned data B
pub type DACC1DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn daccdhr(&self) -> DACCDHR_R {
        DACCDHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DAC channel1 12-bit left-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> DACC1DHRB_R {
        DACC1DHRB_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12L")
            .field("daccdhr", &self.daccdhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn daccdhr(&mut self) -> DACCDHR_W<DHR12Lrs> {
        DACCDHR_W::new(self, 4)
    }
    ///Bits 20:31 - DAC channel1 12-bit left-aligned data B
    #[inline(always)]
    pub fn dacc1dhrb(&mut self) -> DACC1DHRB_W<DHR12Lrs> {
        DACC1DHRB_W::new(self, 20)
    }
}
/**channel%s 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DAC1:DHR12L[1])*/
pub struct DHR12Lrs;
impl crate::RegisterSpec for DHR12Lrs {
    type Ux = u32;
}
///`read()` method returns [`dhr12l::R`](R) reader structure
impl crate::Readable for DHR12Lrs {}
///`write(|w| ..)` method takes [`dhr12l::W`](W) writer structure
impl crate::Writable for DHR12Lrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12L%s to value 0
impl crate::Resettable for DHR12Lrs {}
