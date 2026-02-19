///Register `IMR1` reader
pub type R = crate::R<IMR1rs>;
///Register `IMR1` writer
pub type W = crate::W<IMR1rs>;
/**CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `IM0` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub type IM0_R = crate::BitReader<INTERRUPT_MASK>;
impl IM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK {
        match self.bits {
            false => INTERRUPT_MASK::Masked,
            true => INTERRUPT_MASK::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `IM0` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub type IM0_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> IM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
///Field `IM1` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM1_R;
///Field `IM2` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM2_R;
///Field `IM3` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM3_R;
///Field `IM4` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM4_R;
///Field `IM5` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM5_R;
///Field `IM6` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM6_R;
///Field `IM7` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM7_R;
///Field `IM8` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM8_R;
///Field `IM9` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM9_R;
///Field `IM10` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM10_R;
///Field `IM11` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM11_R;
///Field `IM12` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM12_R;
///Field `IM13` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM13_R;
///Field `IM14` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM14_R;
///Field `IM15` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM15_R;
///Field `IM16` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM16_R;
///Field `IM17` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM17_R;
///Field `IM18` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM18_R;
///Field `IM19` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM19_R;
///Field `IM20` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM20_R;
///Field `IM21` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM21_R;
///Field `IM22` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM22_R;
///Field `IM23` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM23_R;
///Field `IM24` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM24_R;
///Field `IM25` reader - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_R as IM25_R;
///Field `IM1` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM1_W;
///Field `IM2` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM2_W;
///Field `IM3` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM3_W;
///Field `IM4` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM4_W;
///Field `IM5` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM5_W;
///Field `IM6` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM6_W;
///Field `IM7` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM7_W;
///Field `IM8` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM8_W;
///Field `IM9` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM9_W;
///Field `IM10` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM10_W;
///Field `IM11` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM11_W;
///Field `IM12` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM12_W;
///Field `IM13` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM13_W;
///Field `IM14` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM14_W;
///Field `IM15` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM15_W;
///Field `IM16` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM16_W;
///Field `IM17` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM17_W;
///Field `IM18` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM18_W;
///Field `IM19` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM19_W;
///Field `IM20` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM20_W;
///Field `IM21` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM21_W;
///Field `IM22` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM22_W;
///Field `IM23` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM23_W;
///Field `IM24` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM24_W;
///Field `IM25` writer - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
pub use IM0_W as IM25_W;
impl R {
    ///Bit 0 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im24(&self) -> IM24_R {
        IM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR1")
            .field("im0", &self.im0())
            .field("im1", &self.im1())
            .field("im2", &self.im2())
            .field("im3", &self.im3())
            .field("im4", &self.im4())
            .field("im5", &self.im5())
            .field("im6", &self.im6())
            .field("im7", &self.im7())
            .field("im8", &self.im8())
            .field("im9", &self.im9())
            .field("im10", &self.im10())
            .field("im11", &self.im11())
            .field("im12", &self.im12())
            .field("im13", &self.im13())
            .field("im14", &self.im14())
            .field("im15", &self.im15())
            .field("im16", &self.im16())
            .field("im17", &self.im17())
            .field("im18", &self.im18())
            .field("im19", &self.im19())
            .field("im20", &self.im20())
            .field("im21", &self.im21())
            .field("im22", &self.im22())
            .field("im23", &self.im23())
            .field("im24", &self.im24())
            .field("im25", &self.im25())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W<IMR1rs> {
        IM0_W::new(self, 0)
    }
    ///Bit 1 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W<IMR1rs> {
        IM1_W::new(self, 1)
    }
    ///Bit 2 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W<IMR1rs> {
        IM2_W::new(self, 2)
    }
    ///Bit 3 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W<IMR1rs> {
        IM3_W::new(self, 3)
    }
    ///Bit 4 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W<IMR1rs> {
        IM4_W::new(self, 4)
    }
    ///Bit 5 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W<IMR1rs> {
        IM5_W::new(self, 5)
    }
    ///Bit 6 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W<IMR1rs> {
        IM6_W::new(self, 6)
    }
    ///Bit 7 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W<IMR1rs> {
        IM7_W::new(self, 7)
    }
    ///Bit 8 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W<IMR1rs> {
        IM8_W::new(self, 8)
    }
    ///Bit 9 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W<IMR1rs> {
        IM9_W::new(self, 9)
    }
    ///Bit 10 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W<IMR1rs> {
        IM10_W::new(self, 10)
    }
    ///Bit 11 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W<IMR1rs> {
        IM11_W::new(self, 11)
    }
    ///Bit 12 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W<IMR1rs> {
        IM12_W::new(self, 12)
    }
    ///Bit 13 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W<IMR1rs> {
        IM13_W::new(self, 13)
    }
    ///Bit 14 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W<IMR1rs> {
        IM14_W::new(self, 14)
    }
    ///Bit 15 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W<IMR1rs> {
        IM15_W::new(self, 15)
    }
    ///Bit 16 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W<IMR1rs> {
        IM16_W::new(self, 16)
    }
    ///Bit 17 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im17(&mut self) -> IM17_W<IMR1rs> {
        IM17_W::new(self, 17)
    }
    ///Bit 18 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im18(&mut self) -> IM18_W<IMR1rs> {
        IM18_W::new(self, 18)
    }
    ///Bit 19 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W<IMR1rs> {
        IM19_W::new(self, 19)
    }
    ///Bit 20 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W<IMR1rs> {
        IM20_W::new(self, 20)
    }
    ///Bit 21 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im21(&mut self) -> IM21_W<IMR1rs> {
        IM21_W::new(self, 21)
    }
    ///Bit 22 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im22(&mut self) -> IM22_W<IMR1rs> {
        IM22_W::new(self, 22)
    }
    ///Bit 23 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W<IMR1rs> {
        IM23_W::new(self, 23)
    }
    ///Bit 24 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im24(&mut self) -> IM24_W<IMR1rs> {
        IM24_W::new(self, 24)
    }
    ///Bit 25 - CPU wake-up with interrupt mask on event input x When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access. Non-secure write to this bit is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded. Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series. Refer to the EXTI line connections table for its availability. If not present, consider this bit as reserved and keep at reset value.
    #[inline(always)]
    pub fn im25(&mut self) -> IM25_W<IMR1rs> {
        IM25_W::new(self, 25)
    }
}
/**EXTI CPU wake-up with interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#EXTI:IMR1)*/
pub struct IMR1rs;
impl crate::RegisterSpec for IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`imr1::R`](R) reader structure
impl crate::Readable for IMR1rs {}
///`write(|w| ..)` method takes [`imr1::W`](W) writer structure
impl crate::Writable for IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR1 to value 0
impl crate::Resettable for IMR1rs {}
