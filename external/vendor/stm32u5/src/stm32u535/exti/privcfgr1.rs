///Register `PRIVCFGR1` reader
pub type R = crate::R<PRIVCFGR1rs>;
///Register `PRIVCFGR1` writer
pub type W = crate::W<PRIVCFGR1rs>;
/**Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_PRIVILEGE {
    ///0: Event privilege disabled
    Unprivileged = 0,
    ///1: Event privilege enabled
    Privileged = 1,
}
impl From<EVENT_PRIVILEGE> for bool {
    #[inline(always)]
    fn from(variant: EVENT_PRIVILEGE) -> Self {
        variant as u8 != 0
    }
}
///Field `PRIV0` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub type PRIV0_R = crate::BitReader<EVENT_PRIVILEGE>;
impl PRIV0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_PRIVILEGE {
        match self.bits {
            false => EVENT_PRIVILEGE::Unprivileged,
            true => EVENT_PRIVILEGE::Privileged,
        }
    }
    ///Event privilege disabled
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == EVENT_PRIVILEGE::Unprivileged
    }
    ///Event privilege enabled
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == EVENT_PRIVILEGE::Privileged
    }
}
///Field `PRIV0` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_PRIVILEGE>;
impl<'a, REG> PRIV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event privilege disabled
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_PRIVILEGE::Unprivileged)
    }
    ///Event privilege enabled
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_PRIVILEGE::Privileged)
    }
}
///Field `PRIV1` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV1_R;
///Field `PRIV2` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV2_R;
///Field `PRIV3` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV3_R;
///Field `PRIV4` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV4_R;
///Field `PRIV5` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV5_R;
///Field `PRIV6` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV6_R;
///Field `PRIV7` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV7_R;
///Field `PRIV8` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV8_R;
///Field `PRIV9` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV9_R;
///Field `PRIV10` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV10_R;
///Field `PRIV11` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV11_R;
///Field `PRIV12` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV12_R;
///Field `PRIV13` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV13_R;
///Field `PRIV14` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV14_R;
///Field `PRIV15` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV15_R;
///Field `PRIV16` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV16_R;
///Field `PRIV17` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV17_R;
///Field `PRIV18` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV18_R;
///Field `PRIV19` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV19_R;
///Field `PRIV20` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV20_R;
///Field `PRIV21` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV21_R;
///Field `PRIV22` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV22_R;
///Field `PRIV23` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV23_R;
///Field `PRIV24` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV24_R;
///Field `PRIV25` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_R as PRIV25_R;
///Field `PRIV1` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV1_W;
///Field `PRIV2` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV2_W;
///Field `PRIV3` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV3_W;
///Field `PRIV4` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV4_W;
///Field `PRIV5` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV5_W;
///Field `PRIV6` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV6_W;
///Field `PRIV7` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV7_W;
///Field `PRIV8` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV8_W;
///Field `PRIV9` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV9_W;
///Field `PRIV10` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV10_W;
///Field `PRIV11` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV11_W;
///Field `PRIV12` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV12_W;
///Field `PRIV13` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV13_W;
///Field `PRIV14` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV14_W;
///Field `PRIV15` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV15_W;
///Field `PRIV16` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV16_W;
///Field `PRIV17` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV17_W;
///Field `PRIV18` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV18_W;
///Field `PRIV19` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV19_W;
///Field `PRIV20` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV20_W;
///Field `PRIV21` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV21_W;
///Field `PRIV22` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV22_W;
///Field `PRIV23` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV23_W;
///Field `PRIV24` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV24_W;
///Field `PRIV25` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use PRIV0_W as PRIV25_W;
impl R {
    ///Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR1")
            .field("priv0", &self.priv0())
            .field("priv1", &self.priv1())
            .field("priv2", &self.priv2())
            .field("priv3", &self.priv3())
            .field("priv4", &self.priv4())
            .field("priv5", &self.priv5())
            .field("priv6", &self.priv6())
            .field("priv7", &self.priv7())
            .field("priv8", &self.priv8())
            .field("priv9", &self.priv9())
            .field("priv10", &self.priv10())
            .field("priv11", &self.priv11())
            .field("priv12", &self.priv12())
            .field("priv13", &self.priv13())
            .field("priv14", &self.priv14())
            .field("priv15", &self.priv15())
            .field("priv16", &self.priv16())
            .field("priv17", &self.priv17())
            .field("priv18", &self.priv18())
            .field("priv19", &self.priv19())
            .field("priv20", &self.priv20())
            .field("priv21", &self.priv21())
            .field("priv22", &self.priv22())
            .field("priv23", &self.priv23())
            .field("priv24", &self.priv24())
            .field("priv25", &self.priv25())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV0_W<PRIVCFGR1rs> {
        PRIV0_W::new(self, 0)
    }
    ///Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV1_W<PRIVCFGR1rs> {
        PRIV1_W::new(self, 1)
    }
    ///Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV2_W<PRIVCFGR1rs> {
        PRIV2_W::new(self, 2)
    }
    ///Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV3_W<PRIVCFGR1rs> {
        PRIV3_W::new(self, 3)
    }
    ///Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV4_W<PRIVCFGR1rs> {
        PRIV4_W::new(self, 4)
    }
    ///Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV5_W<PRIVCFGR1rs> {
        PRIV5_W::new(self, 5)
    }
    ///Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV6_W<PRIVCFGR1rs> {
        PRIV6_W::new(self, 6)
    }
    ///Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV7_W<PRIVCFGR1rs> {
        PRIV7_W::new(self, 7)
    }
    ///Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv8(&mut self) -> PRIV8_W<PRIVCFGR1rs> {
        PRIV8_W::new(self, 8)
    }
    ///Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv9(&mut self) -> PRIV9_W<PRIVCFGR1rs> {
        PRIV9_W::new(self, 9)
    }
    ///Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv10(&mut self) -> PRIV10_W<PRIVCFGR1rs> {
        PRIV10_W::new(self, 10)
    }
    ///Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv11(&mut self) -> PRIV11_W<PRIVCFGR1rs> {
        PRIV11_W::new(self, 11)
    }
    ///Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv12(&mut self) -> PRIV12_W<PRIVCFGR1rs> {
        PRIV12_W::new(self, 12)
    }
    ///Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv13(&mut self) -> PRIV13_W<PRIVCFGR1rs> {
        PRIV13_W::new(self, 13)
    }
    ///Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv14(&mut self) -> PRIV14_W<PRIVCFGR1rs> {
        PRIV14_W::new(self, 14)
    }
    ///Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv15(&mut self) -> PRIV15_W<PRIVCFGR1rs> {
        PRIV15_W::new(self, 15)
    }
    ///Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv16(&mut self) -> PRIV16_W<PRIVCFGR1rs> {
        PRIV16_W::new(self, 16)
    }
    ///Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv17(&mut self) -> PRIV17_W<PRIVCFGR1rs> {
        PRIV17_W::new(self, 17)
    }
    ///Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv18(&mut self) -> PRIV18_W<PRIVCFGR1rs> {
        PRIV18_W::new(self, 18)
    }
    ///Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv19(&mut self) -> PRIV19_W<PRIVCFGR1rs> {
        PRIV19_W::new(self, 19)
    }
    ///Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv20(&mut self) -> PRIV20_W<PRIVCFGR1rs> {
        PRIV20_W::new(self, 20)
    }
    ///Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv21(&mut self) -> PRIV21_W<PRIVCFGR1rs> {
        PRIV21_W::new(self, 21)
    }
    ///Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv22(&mut self) -> PRIV22_W<PRIVCFGR1rs> {
        PRIV22_W::new(self, 22)
    }
    ///Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv23(&mut self) -> PRIV23_W<PRIVCFGR1rs> {
        PRIV23_W::new(self, 23)
    }
    ///Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv24(&mut self) -> PRIV24_W<PRIVCFGR1rs> {
        PRIV24_W::new(self, 24)
    }
    ///Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded. Note: PRIV25, PRIV24, and PRIV23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn priv25(&mut self) -> PRIV25_W<PRIVCFGR1rs> {
        PRIV25_W::new(self, 25)
    }
}
/**EXTI privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#EXTI:PRIVCFGR1)*/
pub struct PRIVCFGR1rs;
impl crate::RegisterSpec for PRIVCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr1::R`](R) reader structure
impl crate::Readable for PRIVCFGR1rs {}
///`write(|w| ..)` method takes [`privcfgr1::W`](W) writer structure
impl crate::Writable for PRIVCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR1 to value 0
impl crate::Resettable for PRIVCFGR1rs {}
