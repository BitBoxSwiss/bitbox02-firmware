///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `YEMPTY` reader - Y buffer empty flag
pub type YEMPTY_R = crate::BitReader;
///Field `X1FULL` reader - X1 buffer full flag
pub type X1FULL_R = crate::BitReader;
///Field `OVFL` reader - Overflow error flag
pub type OVFL_R = crate::BitReader;
///Field `UNFL` reader - Underflow error flag
pub type UNFL_R = crate::BitReader;
///Field `SAT` reader - Saturation error flag
pub type SAT_R = crate::BitReader;
impl R {
    ///Bit 0 - Y buffer empty flag
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - X1 buffer full flag
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Overflow error flag
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Underflow error flag
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Saturation error flag
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("yempty", &self.yempty())
            .field("x1full", &self.x1full())
            .field("ovfl", &self.ovfl())
            .field("unfl", &self.unfl())
            .field("sat", &self.sat())
            .finish()
    }
}
/**FMAC Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FMAC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
