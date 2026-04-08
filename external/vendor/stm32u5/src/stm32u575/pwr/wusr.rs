///Register `WUSR` reader
pub type R = crate::R<WUSRrs>;
/**Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1 = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1 {
    ///0: No wakeup event occurred on WKUPx pin
    NoWakeup = 0,
    ///1: A wakeup event occurred on WKUPx pin
    Wakeup = 1,
}
impl From<WUF1> for bool {
    #[inline(always)]
    fn from(variant: WUF1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF1` reader - Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1 = 0.
pub type WUF1_R = crate::BitReader<WUF1>;
impl WUF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF1 {
        match self.bits {
            false => WUF1::NoWakeup,
            true => WUF1::Wakeup,
        }
    }
    ///No wakeup event occurred on WKUPx pin
    #[inline(always)]
    pub fn is_no_wakeup(&self) -> bool {
        *self == WUF1::NoWakeup
    }
    ///A wakeup event occurred on WKUPx pin
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF1::Wakeup
    }
}
///Field `WUF2` reader - Wakeup flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2 = 0.
pub use WUF1_R as WUF2_R;
///Field `WUF3` reader - Wakeup flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3 = 0.
pub use WUF1_R as WUF3_R;
///Field `WUF4` reader - Wakeup flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4 = 0.
pub use WUF1_R as WUF4_R;
///Field `WUF5` reader - Wakeup flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5 = 0.
pub use WUF1_R as WUF5_R;
///Field `WUF6` reader - Wakeup flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.
pub use WUF1_R as WUF6_R;
///Field `WUF7` reader - Wakeup flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.
pub use WUF1_R as WUF7_R;
///Field `WUF8` reader - Wakeup flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.
pub use WUF1_R as WUF8_R;
impl R {
    ///Bit 0 - Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1 = 0.
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2 = 0.
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3 = 0.
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4 = 0.
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5 = 0.
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared.
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUSR")
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("wuf6", &self.wuf6())
            .field("wuf7", &self.wuf7())
            .field("wuf8", &self.wuf8())
            .finish()
    }
}
/**PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`wusr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:WUSR)*/
pub struct WUSRrs;
impl crate::RegisterSpec for WUSRrs {
    type Ux = u32;
}
///`read()` method returns [`wusr::R`](R) reader structure
impl crate::Readable for WUSRrs {}
///`reset()` method sets WUSR to value 0
impl crate::Resettable for WUSRrs {}
