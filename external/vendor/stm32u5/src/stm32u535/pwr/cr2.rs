///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1PDS1 {
    ///0: SRAM1 page x content retained in Stop modes
    Disabled = 0,
    ///1: SRAM1 page x content lost in Stop modes
    Enabled = 1,
}
impl From<SRAM1PDS1> for bool {
    #[inline(always)]
    fn from(variant: SRAM1PDS1) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM1PDS1` reader - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS1_R = crate::BitReader<SRAM1PDS1>;
impl SRAM1PDS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1PDS1 {
        match self.bits {
            false => SRAM1PDS1::Disabled,
            true => SRAM1PDS1::Enabled,
        }
    }
    ///SRAM1 page x content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM1PDS1::Disabled
    }
    ///SRAM1 page x content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM1PDS1::Enabled
    }
}
///Field `SRAM1PDS1` writer - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM1PDS1_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1PDS1>;
impl<'a, REG> SRAM1PDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM1 page x content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1PDS1::Disabled)
    }
    ///SRAM1 page x content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1PDS1::Enabled)
    }
}
///Field `SRAM1PDS2` reader - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub use SRAM1PDS1_R as SRAM1PDS2_R;
///Field `SRAM1PDS3` reader - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub use SRAM1PDS1_R as SRAM1PDS3_R;
///Field `SRAM1PDS2` writer - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub use SRAM1PDS1_W as SRAM1PDS2_W;
///Field `SRAM1PDS3` writer - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
pub use SRAM1PDS1_W as SRAM1PDS3_W;
/**SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2PDS1 {
    ///0: SRAM2 page x content retained in Stop modes
    Disabled = 0,
    ///1: SRAM2 page x content lost in Stop modes
    Enabled = 1,
}
impl From<SRAM2PDS1> for bool {
    #[inline(always)]
    fn from(variant: SRAM2PDS1) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2PDS1` reader - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
pub type SRAM2PDS1_R = crate::BitReader<SRAM2PDS1>;
impl SRAM2PDS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2PDS1 {
        match self.bits {
            false => SRAM2PDS1::Disabled,
            true => SRAM2PDS1::Enabled,
        }
    }
    ///SRAM2 page x content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM2PDS1::Disabled
    }
    ///SRAM2 page x content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM2PDS1::Enabled
    }
}
///Field `SRAM2PDS1` writer - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
pub type SRAM2PDS1_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2PDS1>;
impl<'a, REG> SRAM2PDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 page x content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2PDS1::Disabled)
    }
    ///SRAM2 page x content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2PDS1::Enabled)
    }
}
///Field `SRAM2PDS2` reader - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
pub use SRAM2PDS1_R as SRAM2PDS2_R;
///Field `SRAM2PDS2` writer - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
pub use SRAM2PDS1_W as SRAM2PDS2_W;
/**SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM4PDS {
    ///0: SRAM4 content retained in Stop modes
    Disabled = 0,
    ///1: SRAM4 content lost in Stop modes
    Enabled = 1,
}
impl From<SRAM4PDS> for bool {
    #[inline(always)]
    fn from(variant: SRAM4PDS) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM4PDS` reader - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM4PDS_R = crate::BitReader<SRAM4PDS>;
impl SRAM4PDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM4PDS {
        match self.bits {
            false => SRAM4PDS::Disabled,
            true => SRAM4PDS::Enabled,
        }
    }
    ///SRAM4 content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM4PDS::Disabled
    }
    ///SRAM4 content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM4PDS::Enabled
    }
}
///Field `SRAM4PDS` writer - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
pub type SRAM4PDS_W<'a, REG> = crate::BitWriter<'a, REG, SRAM4PDS>;
impl<'a, REG> SRAM4PDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM4 content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM4PDS::Disabled)
    }
    ///SRAM4 content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM4PDS::Enabled)
    }
}
/**ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRAMPDS {
    ///0: ICACHE SRAM content retained in Stop modes
    Disabled = 0,
    ///1: ICACHE SRAM content lost in Stop modes
    Enabled = 1,
}
impl From<ICRAMPDS> for bool {
    #[inline(always)]
    fn from(variant: ICRAMPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `ICRAMPDS` reader - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type ICRAMPDS_R = crate::BitReader<ICRAMPDS>;
impl ICRAMPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICRAMPDS {
        match self.bits {
            false => ICRAMPDS::Disabled,
            true => ICRAMPDS::Enabled,
        }
    }
    ///ICACHE SRAM content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICRAMPDS::Disabled
    }
    ///ICACHE SRAM content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICRAMPDS::Enabled
    }
}
///Field `ICRAMPDS` writer - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type ICRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG, ICRAMPDS>;
impl<'a, REG> ICRAMPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ICACHE SRAM content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICRAMPDS::Disabled)
    }
    ///ICACHE SRAM content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ICRAMPDS::Enabled)
    }
}
/**DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC1RAMPDS {
    ///0: DCACHE1 SRAM content retained in Stop modes
    Disabled = 0,
    ///1: DCACHE1 SRAM content lost in Stop modes
    Enabled = 1,
}
impl From<DC1RAMPDS> for bool {
    #[inline(always)]
    fn from(variant: DC1RAMPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `DC1RAMPDS` reader - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DC1RAMPDS_R = crate::BitReader<DC1RAMPDS>;
impl DC1RAMPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DC1RAMPDS {
        match self.bits {
            false => DC1RAMPDS::Disabled,
            true => DC1RAMPDS::Enabled,
        }
    }
    ///DCACHE1 SRAM content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DC1RAMPDS::Disabled
    }
    ///DCACHE1 SRAM content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DC1RAMPDS::Enabled
    }
}
///Field `DC1RAMPDS` writer - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type DC1RAMPDS_W<'a, REG> = crate::BitWriter<'a, REG, DC1RAMPDS>;
impl<'a, REG> DC1RAMPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DCACHE1 SRAM content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DC1RAMPDS::Disabled)
    }
    ///DCACHE1 SRAM content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DC1RAMPDS::Enabled)
    }
}
/**FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRAMPDS {
    ///0: FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content retained in Stop modes
    Disabled = 0,
    ///1: FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content lost in Stop modes
    Enabled = 1,
}
impl From<PRAMPDS> for bool {
    #[inline(always)]
    fn from(variant: PRAMPDS) -> Self {
        variant as u8 != 0
    }
}
///Field `PRAMPDS` reader - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type PRAMPDS_R = crate::BitReader<PRAMPDS>;
impl PRAMPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRAMPDS {
        match self.bits {
            false => PRAMPDS::Disabled,
            true => PRAMPDS::Enabled,
        }
    }
    ///FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRAMPDS::Disabled
    }
    ///FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRAMPDS::Enabled
    }
}
///Field `PRAMPDS` writer - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
pub type PRAMPDS_W<'a, REG> = crate::BitWriter<'a, REG, PRAMPDS>;
impl<'a, REG> PRAMPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRAMPDS::Disabled)
    }
    ///FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRAMPDS::Enabled)
    }
}
/**SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM4FWU {
    ///0: SRAM4 enters low-power mode in Stop 0/1/2 modes (source biasing for lower-power consumption)
    Disabled = 0,
    ///1: SRAM4 remains in normal mode in Stop 0/1/2 modes (higher consumption but no SRAM4 wake-up time)
    Enabled = 1,
}
impl From<SRAM4FWU> for bool {
    #[inline(always)]
    fn from(variant: SRAM4FWU) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM4FWU` reader - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
pub type SRAM4FWU_R = crate::BitReader<SRAM4FWU>;
impl SRAM4FWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM4FWU {
        match self.bits {
            false => SRAM4FWU::Disabled,
            true => SRAM4FWU::Enabled,
        }
    }
    ///SRAM4 enters low-power mode in Stop 0/1/2 modes (source biasing for lower-power consumption)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM4FWU::Disabled
    }
    ///SRAM4 remains in normal mode in Stop 0/1/2 modes (higher consumption but no SRAM4 wake-up time)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM4FWU::Enabled
    }
}
///Field `SRAM4FWU` writer - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
pub type SRAM4FWU_W<'a, REG> = crate::BitWriter<'a, REG, SRAM4FWU>;
impl<'a, REG> SRAM4FWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM4 enters low-power mode in Stop 0/1/2 modes (source biasing for lower-power consumption)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM4FWU::Disabled)
    }
    ///SRAM4 remains in normal mode in Stop 0/1/2 modes (higher consumption but no SRAM4 wake-up time)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM4FWU::Enabled)
    }
}
/**Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHFWU {
    ///0: Flash memory enters low-power mode in Stop 0/1 modes (lower-power consumption)
    Disabled = 0,
    ///1: Flash memory remains in normal mode in Stop 0/1 modes (faster wake-up time)
    Enabled = 1,
}
impl From<FLASHFWU> for bool {
    #[inline(always)]
    fn from(variant: FLASHFWU) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHFWU` reader - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_R = crate::BitReader<FLASHFWU>;
impl FLASHFWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHFWU {
        match self.bits {
            false => FLASHFWU::Disabled,
            true => FLASHFWU::Enabled,
        }
    }
    ///Flash memory enters low-power mode in Stop 0/1 modes (lower-power consumption)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHFWU::Disabled
    }
    ///Flash memory remains in normal mode in Stop 0/1 modes (faster wake-up time)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHFWU::Enabled
    }
}
///Field `FLASHFWU` writer - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
pub type FLASHFWU_W<'a, REG> = crate::BitWriter<'a, REG, FLASHFWU>;
impl<'a, REG> FLASHFWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory enters low-power mode in Stop 0/1 modes (lower-power consumption)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHFWU::Disabled)
    }
    ///Flash memory remains in normal mode in Stop 0/1 modes (faster wake-up time)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHFWU::Enabled)
    }
}
/**SmartRun domain in Run mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRDRUN {
    ///0: SmartRun domain AHB3 and APB3 clocks disabled by default in Stop 0/1/2 modes
    Disabled = 0,
    ///1: SmartRun domain AHB3 and APB3 clocks kept enabled in Stop 0/1/2 modes
    Enabled = 1,
}
impl From<SRDRUN> for bool {
    #[inline(always)]
    fn from(variant: SRDRUN) -> Self {
        variant as u8 != 0
    }
}
///Field `SRDRUN` reader - SmartRun domain in Run mode
pub type SRDRUN_R = crate::BitReader<SRDRUN>;
impl SRDRUN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRDRUN {
        match self.bits {
            false => SRDRUN::Disabled,
            true => SRDRUN::Enabled,
        }
    }
    ///SmartRun domain AHB3 and APB3 clocks disabled by default in Stop 0/1/2 modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRDRUN::Disabled
    }
    ///SmartRun domain AHB3 and APB3 clocks kept enabled in Stop 0/1/2 modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRDRUN::Enabled
    }
}
///Field `SRDRUN` writer - SmartRun domain in Run mode
pub type SRDRUN_W<'a, REG> = crate::BitWriter<'a, REG, SRDRUN>;
impl<'a, REG> SRDRUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SmartRun domain AHB3 and APB3 clocks disabled by default in Stop 0/1/2 modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRDRUN::Disabled)
    }
    ///SmartRun domain AHB3 and APB3 clocks kept enabled in Stop 0/1/2 modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRDRUN::Enabled)
    }
}
impl R {
    ///Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds1(&self) -> SRAM1PDS1_R {
        SRAM1PDS1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds2(&self) -> SRAM1PDS2_R {
        SRAM1PDS2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds3(&self) -> SRAM1PDS3_R {
        SRAM1PDS3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds1(&self) -> SRAM2PDS1_R {
        SRAM2PDS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds2(&self) -> SRAM2PDS2_R {
        SRAM2PDS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram4pds(&self) -> SRAM4PDS_R {
        SRAM4PDS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn icrampds(&self) -> ICRAMPDS_R {
        ICRAMPDS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn dc1rampds(&self) -> DC1RAMPDS_R {
        DC1RAMPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn prampds(&self) -> PRAMPDS_R {
        PRAMPDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
    #[inline(always)]
    pub fn sram4fwu(&self) -> SRAM4FWU_R {
        SRAM4FWU_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    pub fn flashfwu(&self) -> FLASHFWU_R {
        FLASHFWU_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 31 - SmartRun domain in Run mode
    #[inline(always)]
    pub fn srdrun(&self) -> SRDRUN_R {
        SRDRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("sram1pds1", &self.sram1pds1())
            .field("sram1pds2", &self.sram1pds2())
            .field("sram1pds3", &self.sram1pds3())
            .field("sram2pds1", &self.sram2pds1())
            .field("sram2pds2", &self.sram2pds2())
            .field("sram4pds", &self.sram4pds())
            .field("icrampds", &self.icrampds())
            .field("dc1rampds", &self.dc1rampds())
            .field("prampds", &self.prampds())
            .field("sram4fwu", &self.sram4fwu())
            .field("flashfwu", &self.flashfwu())
            .field("srdrun", &self.srdrun())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM1 page 1 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds1(&mut self) -> SRAM1PDS1_W<CR2rs> {
        SRAM1PDS1_W::new(self, 0)
    }
    ///Bit 1 - SRAM1 page 2 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds2(&mut self) -> SRAM1PDS2_W<CR2rs> {
        SRAM1PDS2_W::new(self, 1)
    }
    ///Bit 2 - SRAM1 page 3 (64 Kbytes) power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram1pds3(&mut self) -> SRAM1PDS3_W<CR2rs> {
        SRAM1PDS3_W::new(self, 2)
    }
    ///Bit 4 - SRAM2 page 1 (8 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds1(&mut self) -> SRAM2PDS1_W<CR2rs> {
        SRAM2PDS1_W::new(self, 4)
    }
    ///Bit 5 - SRAM2 page 2 (56 Kbytes) power-down in Stop modes (Stop 0, 1, 2) Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
    #[inline(always)]
    pub fn sram2pds2(&mut self) -> SRAM2PDS2_W<CR2rs> {
        SRAM2PDS2_W::new(self, 5)
    }
    ///Bit 6 - SRAM4 power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn sram4pds(&mut self) -> SRAM4PDS_W<CR2rs> {
        SRAM4PDS_W::new(self, 6)
    }
    ///Bit 8 - ICACHE SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn icrampds(&mut self) -> ICRAMPDS_W<CR2rs> {
        ICRAMPDS_W::new(self, 8)
    }
    ///Bit 9 - DCACHE1 SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn dc1rampds(&mut self) -> DC1RAMPDS_W<CR2rs> {
        DC1RAMPDS_W::new(self, 9)
    }
    ///Bit 11 - FMAC, FDCAN and USB peripherals SRAM power-down in Stop modes (Stop 0, 1, 2, 3)
    #[inline(always)]
    pub fn prampds(&mut self) -> PRAMPDS_W<CR2rs> {
        PRAMPDS_W::new(self, 11)
    }
    ///Bit 13 - SRAM4 fast wakeup from Stop 0, Stop 1 and Stop 2 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time. SRAM4 wakeup time increases the wakeup time when exiting Stop 0, 1 and 2 modes, and also increases the LPDMA access time to SRAM4 during Stop modes.
    #[inline(always)]
    pub fn sram4fwu(&mut self) -> SRAM4FWU_W<CR2rs> {
        SRAM4FWU_W::new(self, 13)
    }
    ///Bit 14 - Flash memory fast wakeup from Stop 0 and Stop 1 modes This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes. When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.
    #[inline(always)]
    pub fn flashfwu(&mut self) -> FLASHFWU_W<CR2rs> {
        FLASHFWU_W::new(self, 14)
    }
    ///Bit 31 - SmartRun domain in Run mode
    #[inline(always)]
    pub fn srdrun(&mut self) -> SRDRUN_W<CR2rs> {
        SRDRUN_W::new(self, 31)
    }
}
/**PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
