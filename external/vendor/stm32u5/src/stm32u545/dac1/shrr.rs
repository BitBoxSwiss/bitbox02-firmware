///Register `SHRR` reader
pub type R = crate::R<SHRRrs>;
///Register `SHRR` writer
pub type W = crate::W<SHRRrs>;
///Field `TREFRESH(1-2)` reader - DAC channel%s refresh time (only valid in Sample and hold mode)
pub type TREFRESH_R = crate::FieldReader;
///Field `TREFRESH(1-2)` writer - DAC channel%s refresh time (only valid in Sample and hold mode)
pub type TREFRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///DAC channel(1-2) refresh time (only valid in Sample and hold mode)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TREFRESH1` field.</div>
    #[inline(always)]
    pub fn trefresh(&self, n: u8) -> TREFRESH_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TREFRESH_R::new(((self.bits >> (n * 16)) & 0xff) as u8)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) refresh time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn trefresh_iter(&self) -> impl Iterator<Item = TREFRESH_R> + '_ {
        (0..2).map(move |n| TREFRESH_R::new(((self.bits >> (n * 16)) & 0xff) as u8))
    }
    ///Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH_R {
        TREFRESH_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - DAC channel2 refresh time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH_R {
        TREFRESH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHRR")
            .field("trefresh1", &self.trefresh1())
            .field("trefresh2", &self.trefresh2())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) refresh time (only valid in Sample and hold mode)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TREFRESH1` field.</div>
    #[inline(always)]
    pub fn trefresh(&mut self, n: u8) -> TREFRESH_W<SHRRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TREFRESH_W::new(self, n * 16)
    }
    ///Bits 0:7 - DAC channel1 refresh time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH_W<SHRRrs> {
        TREFRESH_W::new(self, 0)
    }
    ///Bits 16:23 - DAC channel2 refresh time (only valid in Sample and hold mode)
    #[inline(always)]
    pub fn trefresh2(&mut self) -> TREFRESH_W<SHRRrs> {
        TREFRESH_W::new(self, 16)
    }
}
/**DAC Sample and Hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`shrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DAC1:SHRR)*/
pub struct SHRRrs;
impl crate::RegisterSpec for SHRRrs {
    type Ux = u32;
}
///`read()` method returns [`shrr::R`](R) reader structure
impl crate::Readable for SHRRrs {}
///`write(|w| ..)` method takes [`shrr::W`](W) writer structure
impl crate::Writable for SHRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHRR to value 0x0001_0001
impl crate::Resettable for SHRRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
