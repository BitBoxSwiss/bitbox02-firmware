///Register `CFGLOCK` reader
pub type R = crate::R<CFGLOCKrs>;
///Register `CFGLOCK` writer
pub type W = crate::W<CFGLOCKrs>;
///Field `SPLCK(0-0)` reader - Security/privilege configuration lock for super-block %s
pub type SPLCK_R = crate::BitReader;
///Field `SPLCK(0-0)` writer - Security/privilege configuration lock for super-block %s
pub type SPLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Security/privilege configuration lock for super-block (0-0)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK0` field.</div>
    #[inline(always)]
    pub fn splck(&self, n: u8) -> SPLCK_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        SPLCK_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Security/privilege configuration lock for super-block (0-0)
    #[inline(always)]
    pub fn splck_iter(&self) -> impl Iterator<Item = SPLCK_R> + '_ {
        (0..1).map(move |n| SPLCK_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    ///Bit 0 - Security/privilege configuration lock for super-block 0
    #[inline(always)]
    pub fn splck0(&self) -> SPLCK_R {
        SPLCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGLOCK")
            .field("splck0", &self.splck0())
            .finish()
    }
}
impl W {
    ///Security/privilege configuration lock for super-block (0-0)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK0` field.</div>
    #[inline(always)]
    pub fn splck(&mut self, n: u8) -> SPLCK_W<CFGLOCKrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        SPLCK_W::new(self, n * 0)
    }
    ///Bit 0 - Security/privilege configuration lock for super-block 0
    #[inline(always)]
    pub fn splck0(&mut self) -> SPLCK_W<CFGLOCKrs> {
        SPLCK_W::new(self, 0)
    }
}
/**GTZC2 SRAM4 MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`cfglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC2_MPCBB4:CFGLOCK)*/
pub struct CFGLOCKrs;
impl crate::RegisterSpec for CFGLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`cfglock::R`](R) reader structure
impl crate::Readable for CFGLOCKrs {}
///`write(|w| ..)` method takes [`cfglock::W`](W) writer structure
impl crate::Writable for CFGLOCKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGLOCK to value 0
impl crate::Resettable for CFGLOCKrs {}
