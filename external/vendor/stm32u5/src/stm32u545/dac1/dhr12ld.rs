///Register `DHR12LD` reader
pub type R = crate::R<DHR12LDrs>;
///Register `DHR12LD` writer
pub type W = crate::W<DHR12LDrs>;
///Field `DACCDHR(1-2)` reader - DAC channel%s 12-bit left-aligned data
pub type DACCDHR_R = crate::FieldReader<u16>;
///Field `DACCDHR(1-2)` writer - DAC channel%s 12-bit left-aligned data
pub type DACCDHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///DAC channel(1-2) 12-bit left-aligned data
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DACC1DHR` field.</div>
    #[inline(always)]
    pub fn daccdhr(&self, n: u8) -> DACCDHR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DACCDHR_R::new(((self.bits >> (n * 16 + 4)) & 0x0fff) as u16)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) 12-bit left-aligned data
    #[inline(always)]
    pub fn daccdhr_iter(&self) -> impl Iterator<Item = DACCDHR_R> + '_ {
        (0..2).map(move |n| DACCDHR_R::new(((self.bits >> (n * 16 + 4)) & 0x0fff) as u16))
    }
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACCDHR_R {
        DACCDHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACCDHR_R {
        DACCDHR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHR12LD")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) 12-bit left-aligned data
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DACC1DHR` field.</div>
    #[inline(always)]
    pub fn daccdhr(&mut self, n: u8) -> DACCDHR_W<DHR12LDrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DACCDHR_W::new(self, n * 16 + 4)
    }
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACCDHR_W<DHR12LDrs> {
        DACCDHR_W::new(self, 4)
    }
    ///Bits 20:31 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACCDHR_W<DHR12LDrs> {
        DACCDHR_W::new(self, 20)
    }
}
/**DUAL DAC 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:DHR12LD)*/
pub struct DHR12LDrs;
impl crate::RegisterSpec for DHR12LDrs {
    type Ux = u32;
}
///`read()` method returns [`dhr12ld::R`](R) reader structure
impl crate::Readable for DHR12LDrs {}
///`write(|w| ..)` method takes [`dhr12ld::W`](W) writer structure
impl crate::Writable for DHR12LDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DHR12LD to value 0
impl crate::Resettable for DHR12LDrs {}
