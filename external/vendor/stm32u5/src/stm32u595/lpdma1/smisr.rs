///Register `SMISR` reader
pub type R = crate::R<SMISRrs>;
///Field `MIS(0-3)` reader - MIS%s
pub type MIS_R = crate::BitReader;
impl R {
    ///MIS(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MIS0` field.</div>
    #[inline(always)]
    pub fn mis(&self, n: u8) -> MIS_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        MIS_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///MIS(0-3)
    #[inline(always)]
    pub fn mis_iter(&self) -> impl Iterator<Item = MIS_R> + '_ {
        (0..4).map(move |n| MIS_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - MIS0
    #[inline(always)]
    pub fn mis0(&self) -> MIS_R {
        MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MIS1
    #[inline(always)]
    pub fn mis1(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MIS2
    #[inline(always)]
    pub fn mis2(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MIS3
    #[inline(always)]
    pub fn mis3(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMISR")
            .field("mis0", &self.mis0())
            .field("mis1", &self.mis1())
            .field("mis2", &self.mis2())
            .field("mis3", &self.mis3())
            .finish()
    }
}
/**LPDMA secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:SMISR)*/
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
///`read()` method returns [`smisr::R`](R) reader structure
impl crate::Readable for SMISRrs {}
///`reset()` method sets SMISR to value 0
impl crate::Resettable for SMISRrs {}
