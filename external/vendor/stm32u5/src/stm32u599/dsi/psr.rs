///Register `PSR` reader
pub type R = crate::R<PSRrs>;
///Field `PD` reader - PHY direction This bit indicates the status of phydirection D-PHY signal.
pub type PD_R = crate::BitReader;
///Field `PSSC` reader - PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal.
pub type PSSC_R = crate::BitReader;
///Field `UANC` reader - ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal.
pub type UANC_R = crate::BitReader;
///Field `PSS0` reader - PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal.
pub type PSS0_R = crate::BitReader;
///Field `UAN0` reader - ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal.
pub type UAN0_R = crate::BitReader;
///Field `RUE0` reader - RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal.
pub type RUE0_R = crate::BitReader;
///Field `PSS1` reader - PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal.
pub type PSS1_R = crate::BitReader;
///Field `UAN1` reader - ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal.
pub type UAN1_R = crate::BitReader;
impl R {
    ///Bit 1 - PHY direction This bit indicates the status of phydirection D-PHY signal.
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal.
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal.
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal.
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal.
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal.
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal.
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal.
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("pd", &self.pd())
            .field("pssc", &self.pssc())
            .field("uanc", &self.uanc())
            .field("pss0", &self.pss0())
            .field("uan0", &self.uan0())
            .field("rue0", &self.rue0())
            .field("pss1", &self.pss1())
            .field("uan1", &self.uan1())
            .finish()
    }
}
/**DSI Host PHY status register

You can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:PSR)*/
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
///`read()` method returns [`psr::R`](R) reader structure
impl crate::Readable for PSRrs {}
///`reset()` method sets PSR to value 0x1528
impl crate::Resettable for PSRrs {
    const RESET_VALUE: u32 = 0x1528;
}
