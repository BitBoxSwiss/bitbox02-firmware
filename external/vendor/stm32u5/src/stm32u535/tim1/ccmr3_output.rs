///Register `CCMR3_Output` reader
pub type R = crate::R<CCMR3_OUTPUTrs>;
///Register `CCMR3_Output` writer
pub type W = crate::W<CCMR3_OUTPUTrs>;
///Output compare %s clear enable
pub use super::ccmr1_output::OC1CE;
///Output compare %s fast enable
pub use super::ccmr1_output::OC1FE;
///Output compare %s mode
pub use super::ccmr1_output::OC1M;
///Output compare %s mode, bit 3
pub use super::ccmr1_output::OC1M_3;
///Output compare %s preload enable
pub use super::ccmr1_output::OC1PE;
///Field `OCCE(5-6)` reader - Output compare %s clear enable
pub use super::ccmr1_output::OCCE_R;
///Field `OCCE(5-6)` writer - Output compare %s clear enable
pub use super::ccmr1_output::OCCE_W;
///Field `OCFE(5-6)` reader - Output compare %s fast enable
pub use super::ccmr1_output::OCFE_R;
///Field `OCFE(5-6)` writer - Output compare %s fast enable
pub use super::ccmr1_output::OCFE_W;
///Field `OCM_3(5-6)` reader - Output compare %s mode, bit 3
pub use super::ccmr1_output::OCM_3_R;
///Field `OCM_3(5-6)` writer - Output compare %s mode, bit 3
pub use super::ccmr1_output::OCM_3_W;
///Field `OCM(5-6)` reader - Output compare %s mode
pub use super::ccmr1_output::OCM_R;
///Field `OCM(5-6)` writer - Output compare %s mode
pub use super::ccmr1_output::OCM_W;
///Field `OCPE(5-6)` reader - Output compare %s preload enable
pub use super::ccmr1_output::OCPE_R;
///Field `OCPE(5-6)` writer - Output compare %s preload enable
pub use super::ccmr1_output::OCPE_W;
impl R {
    ///Output compare (5-6) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (5-6) fast enable
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..2).map(move |n| OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    ///Bit 2 - Output compare 5 fast enable
    #[inline(always)]
    pub fn oc5fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - Output compare 6 fast enable
    #[inline(always)]
    pub fn oc6fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Output compare (5-6) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (5-6) preload enable
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..2).map(move |n| OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    ///Bit 3 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Output compare (5-6) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5M` field.</div>
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Output compare (5-6) mode
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..2).map(move |n| OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    ///Bits 4:6 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Output compare (5-6) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5CE` field.</div>
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (5-6) clear enable
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..2).map(move |n| OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    ///Bit 7 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Output compare (5-6) mode, bit 3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5M_3` field.</div>
    #[inline(always)]
    pub fn ocm_3(&self, n: u8) -> OCM_3_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (5-6) mode, bit 3
    #[inline(always)]
    pub fn ocm_3_iter(&self) -> impl Iterator<Item = OCM_3_R> + '_ {
        (0..2).map(move |n| OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0))
    }
    ///Bit 16 - Output compare 5 mode, bit 3
    #[inline(always)]
    pub fn oc5m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output compare 6 mode, bit 3
    #[inline(always)]
    pub fn oc6m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR3_Output")
            .field("oc5fe", &self.oc5fe())
            .field("oc6fe", &self.oc6fe())
            .field("oc5pe", &self.oc5pe())
            .field("oc6pe", &self.oc6pe())
            .field("oc5m", &self.oc5m())
            .field("oc6m", &self.oc6m())
            .field("oc5ce", &self.oc5ce())
            .field("oc6ce", &self.oc6ce())
            .field("oc5m_3", &self.oc5m_3())
            .field("oc6m_3", &self.oc6m_3())
            .finish()
    }
}
impl W {
    ///Output compare (5-6) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_W::new(self, n * 8 + 2)
    }
    ///Bit 2 - Output compare 5 fast enable
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OCFE_W<CCMR3_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    ///Bit 10 - Output compare 6 fast enable
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OCFE_W<CCMR3_OUTPUTrs> {
        OCFE_W::new(self, 10)
    }
    ///Output compare (5-6) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_W::new(self, n * 8 + 3)
    }
    ///Bit 3 - Output compare 5 preload enable
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OCPE_W<CCMR3_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    ///Bit 11 - Output compare 6 preload enable
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OCPE_W<CCMR3_OUTPUTrs> {
        OCPE_W::new(self, 11)
    }
    ///Output compare (5-6) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5M` field.</div>
    #[inline(always)]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_W::new(self, n * 8 + 4)
    }
    ///Bits 4:6 - Output compare 5 mode
    #[inline(always)]
    pub fn oc5m(&mut self) -> OCM_W<CCMR3_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    ///Bits 12:14 - Output compare 6 mode
    #[inline(always)]
    pub fn oc6m(&mut self) -> OCM_W<CCMR3_OUTPUTrs> {
        OCM_W::new(self, 12)
    }
    ///Output compare (5-6) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5CE` field.</div>
    #[inline(always)]
    pub fn occe(&mut self, n: u8) -> OCCE_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_W::new(self, n * 8 + 7)
    }
    ///Bit 7 - Output compare 5 clear enable
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OCCE_W<CCMR3_OUTPUTrs> {
        OCCE_W::new(self, 7)
    }
    ///Bit 15 - Output compare 6 clear enable
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OCCE_W<CCMR3_OUTPUTrs> {
        OCCE_W::new(self, 15)
    }
    ///Output compare (5-6) mode, bit 3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC5M_3` field.</div>
    #[inline(always)]
    pub fn ocm_3(&mut self, n: u8) -> OCM_3_W<CCMR3_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_W::new(self, n * 8 + 16)
    }
    ///Bit 16 - Output compare 5 mode, bit 3
    #[inline(always)]
    pub fn oc5m_3(&mut self) -> OCM_3_W<CCMR3_OUTPUTrs> {
        OCM_3_W::new(self, 16)
    }
    ///Bit 24 - Output compare 6 mode, bit 3
    #[inline(always)]
    pub fn oc6m_3(&mut self) -> OCM_3_W<CCMR3_OUTPUTrs> {
        OCM_3_W::new(self, 24)
    }
}
/**capture/compare mode register 3

You can [`read`](crate::Reg::read) this register and get [`ccmr3_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TIM1:CCMR3_Output)*/
pub struct CCMR3_OUTPUTrs;
impl crate::RegisterSpec for CCMR3_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr3_output::R`](R) reader structure
impl crate::Readable for CCMR3_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr3_output::W`](W) writer structure
impl crate::Writable for CCMR3_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR3_Output to value 0
impl crate::Resettable for CCMR3_OUTPUTrs {}
