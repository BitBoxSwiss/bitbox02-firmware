///Register `SHHR` reader
pub type R = crate::R<SHHRrs>;
///Register `SHHR` writer
pub type W = crate::W<SHHRrs>;
///Field `THOLD(1-2)` reader - DAC channel%s hold time (only valid in Sample and hold mode)
pub type THOLD_R = crate::FieldReader<u16>;
///Field `THOLD(1-2)` writer - DAC channel%s hold time (only valid in Sample and hold mode)
pub type THOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///DAC channel(1-2) hold time (only valid in Sample and hold mode)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `THOLD1` field.</div>
    #[inline(always)]
    pub fn thold(&self, n: u8) -> THOLD_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        THOLD_R::new(((self.bits >> (n * 16)) & 0x03ff) as u16)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) hold time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn thold_iter(&self) -> impl Iterator<Item = THOLD_R> + '_ {
        (0..2).map(move |n| THOLD_R::new(((self.bits >> (n * 16)) & 0x03ff) as u16))
    }
    ///Bits 0:9 - DAC channel1 hold time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn thold1(&self) -> THOLD_R {
        THOLD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - DAC channel2 hold time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn thold2(&self) -> THOLD_R {
        THOLD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHHR")
            .field("thold1", &self.thold1())
            .field("thold2", &self.thold2())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) hold time (only valid in Sample and hold mode)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `THOLD1` field.</div>
    #[inline(always)]
    pub fn thold(&mut self, n: u8) -> THOLD_W<SHHRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        THOLD_W::new(self, n * 16)
    }
    ///Bits 0:9 - DAC channel1 hold time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn thold1(&mut self) -> THOLD_W<SHHRrs> {
        THOLD_W::new(self, 0)
    }
    ///Bits 16:25 - DAC channel2 hold time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn thold2(&mut self) -> THOLD_W<SHHRrs> {
        THOLD_W::new(self, 16)
    }
}
/**DAC Sample and Hold hold time register

You can [`read`](crate::Reg::read) this register and get [`shhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DAC1:SHHR)*/
pub struct SHHRrs;
impl crate::RegisterSpec for SHHRrs {
    type Ux = u32;
}
///`read()` method returns [`shhr::R`](R) reader structure
impl crate::Readable for SHHRrs {}
///`write(|w| ..)` method takes [`shhr::W`](W) writer structure
impl crate::Writable for SHHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHHR to value 0x0001_0001
impl crate::Resettable for SHHRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
