///Register `CCMR2_Output` reader
pub type R = crate::R<CCMR2_OUTPUTrs>;
///Register `CCMR2_Output` writer
pub type W = crate::W<CCMR2_OUTPUTrs>;
///Capture/Compare %s selection
pub use super::ccmr1_output::CC1S;
///Field `CCS(3-4)` reader - Capture/Compare %s selection
pub use super::ccmr1_output::CCS_R;
///Field `CCS(3-4)` writer - Capture/Compare %s selection
pub use super::ccmr1_output::CCS_W;
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
///Field `OCCE(3-4)` reader - Output compare %s clear enable
pub use super::ccmr1_output::OCCE_R;
///Field `OCCE(3-4)` writer - Output compare %s clear enable
pub use super::ccmr1_output::OCCE_W;
///Field `OCFE(3-4)` reader - Output compare %s fast enable
pub use super::ccmr1_output::OCFE_R;
///Field `OCFE(3-4)` writer - Output compare %s fast enable
pub use super::ccmr1_output::OCFE_W;
///Field `OCM_3(3-4)` reader - Output compare %s mode, bit 3
pub use super::ccmr1_output::OCM_3_R;
///Field `OCM_3(3-4)` writer - Output compare %s mode, bit 3
pub use super::ccmr1_output::OCM_3_W;
///Field `OCM(3-4)` reader - Output compare %s mode
pub use super::ccmr1_output::OCM_R;
///Field `OCM(3-4)` writer - Output compare %s mode
pub use super::ccmr1_output::OCM_W;
///Field `OCPE(3-4)` reader - Output compare %s preload enable
pub use super::ccmr1_output::OCPE_R;
///Field `OCPE(3-4)` writer - Output compare %s preload enable
pub use super::ccmr1_output::OCPE_W;
impl R {
    ///Capture/Compare (3-4) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC3S` field.</div>
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Capture/Compare (3-4) selection
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Output compare (3-4) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&self, n: u8) -> OCFE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (3-4) fast enable
    #[inline(always)]
    pub fn ocfe_iter(&self) -> impl Iterator<Item = OCFE_R> + '_ {
        (0..2).map(move |n| OCFE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&self) -> OCFE_R {
        OCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Output compare (3-4) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&self, n: u8) -> OCPE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (3-4) preload enable
    #[inline(always)]
    pub fn ocpe_iter(&self) -> impl Iterator<Item = OCPE_R> + '_ {
        (0..2).map(move |n| OCPE_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> OCPE_R {
        OCPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Output compare (3-4) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3M` field.</div>
    #[inline(always)]
    pub fn ocm(&self, n: u8) -> OCM_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Output compare (3-4) mode
    #[inline(always)]
    pub fn ocm_iter(&self) -> impl Iterator<Item = OCM_R> + '_ {
        (0..2).map(move |n| OCM_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&self) -> OCM_R {
        OCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Output compare (3-4) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3CE` field.</div>
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (3-4) clear enable
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..2).map(move |n| OCCE_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Output compare (3-4) mode, bit 3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3M_3` field.</div>
    #[inline(always)]
    pub fn ocm_3(&self, n: u8) -> OCM_3_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output compare (3-4) mode, bit 3
    #[inline(always)]
    pub fn ocm_3_iter(&self) -> impl Iterator<Item = OCM_3_R> + '_ {
        (0..2).map(move |n| OCM_3_R::new(((self.bits >> (n * 8 + 16)) & 1) != 0))
    }
    ///Bit 16 - Output compare 3 mode, bit 3
    #[inline(always)]
    pub fn oc3m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Output compare 4 mode, bit 3
    #[inline(always)]
    pub fn oc4m_3(&self) -> OCM_3_R {
        OCM_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Output")
            .field("oc3m_3", &self.oc3m_3())
            .field("oc4m_3", &self.oc4m_3())
            .field("oc3ce", &self.oc3ce())
            .field("oc4ce", &self.oc4ce())
            .field("oc3m", &self.oc3m())
            .field("oc4m", &self.oc4m())
            .field("oc3pe", &self.oc3pe())
            .field("oc4pe", &self.oc4pe())
            .field("oc3fe", &self.oc3fe())
            .field("oc4fe", &self.oc4fe())
            .field("cc3s", &self.cc3s())
            .field("cc4s", &self.cc4s())
            .finish()
    }
}
impl W {
    ///Capture/Compare (3-4) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC3S` field.</div>
    #[inline(always)]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CCS_W<CCMR2_OUTPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CCS_W<CCMR2_OUTPUTrs> {
        CCS_W::new(self, 8)
    }
    ///Output compare (3-4) fast enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3FE` field.</div>
    #[inline(always)]
    pub fn ocfe(&mut self, n: u8) -> OCFE_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCFE_W::new(self, n * 8 + 2)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OCFE_W<CCMR2_OUTPUTrs> {
        OCFE_W::new(self, 2)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OCFE_W<CCMR2_OUTPUTrs> {
        OCFE_W::new(self, 10)
    }
    ///Output compare (3-4) preload enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3PE` field.</div>
    #[inline(always)]
    pub fn ocpe(&mut self, n: u8) -> OCPE_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCPE_W::new(self, n * 8 + 3)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OCPE_W<CCMR2_OUTPUTrs> {
        OCPE_W::new(self, 3)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OCPE_W<CCMR2_OUTPUTrs> {
        OCPE_W::new(self, 11)
    }
    ///Output compare (3-4) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3M` field.</div>
    #[inline(always)]
    pub fn ocm(&mut self, n: u8) -> OCM_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_W::new(self, n * 8 + 4)
    }
    ///Bits 4:6 - Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&mut self) -> OCM_W<CCMR2_OUTPUTrs> {
        OCM_W::new(self, 4)
    }
    ///Bits 12:14 - Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&mut self) -> OCM_W<CCMR2_OUTPUTrs> {
        OCM_W::new(self, 12)
    }
    ///Output compare (3-4) clear enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3CE` field.</div>
    #[inline(always)]
    pub fn occe(&mut self, n: u8) -> OCCE_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCCE_W::new(self, n * 8 + 7)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OCCE_W<CCMR2_OUTPUTrs> {
        OCCE_W::new(self, 7)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OCCE_W<CCMR2_OUTPUTrs> {
        OCCE_W::new(self, 15)
    }
    ///Output compare (3-4) mode, bit 3
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OC3M_3` field.</div>
    #[inline(always)]
    pub fn ocm_3(&mut self, n: u8) -> OCM_3_W<CCMR2_OUTPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OCM_3_W::new(self, n * 8 + 16)
    }
    ///Bit 16 - Output compare 3 mode, bit 3
    #[inline(always)]
    pub fn oc3m_3(&mut self) -> OCM_3_W<CCMR2_OUTPUTrs> {
        OCM_3_W::new(self, 16)
    }
    ///Bit 24 - Output compare 4 mode, bit 3
    #[inline(always)]
    pub fn oc4m_3(&mut self) -> OCM_3_W<CCMR2_OUTPUTrs> {
        OCM_3_W::new(self, 24)
    }
}
/**capture/compare mode register 2 (output mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TIM2:CCMR2_Output)*/
pub struct CCMR2_OUTPUTrs;
impl crate::RegisterSpec for CCMR2_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_output::R`](R) reader structure
impl crate::Readable for CCMR2_OUTPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure
impl crate::Writable for CCMR2_OUTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR2_Output to value 0
impl crate::Resettable for CCMR2_OUTPUTrs {}
