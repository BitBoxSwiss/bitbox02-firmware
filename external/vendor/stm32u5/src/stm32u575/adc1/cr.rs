///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR {
    ///0: ADC is disabled
    Disabled = 0,
    ///1: ADC is enabled
    Enabled = 1,
}
impl From<ADENR> for bool {
    #[inline(always)]
    fn from(variant: ADENR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` reader - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
pub type ADEN_R = crate::BitReader<ADENR>;
impl ADEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADENR {
        match self.bits {
            false => ADENR::Disabled,
            true => ADENR::Enabled,
        }
    }
    ///ADC is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR::Disabled
    }
    ///ADC is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR::Enabled
    }
}
/**ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW {
    ///1: Enable the ADC
    Enabled = 1,
}
impl From<ADENW> for bool {
    #[inline(always)]
    fn from(variant: ADENW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEN` writer - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG, ADENW>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable the ADC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADENW::Enabled)
    }
}
/**ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR {
    ///0: No ADDIS command ongoing
    NotOngoing = 0,
    ///1: An ADDIS command is in progress
    InProgress = 1,
}
impl From<ADDISR> for bool {
    #[inline(always)]
    fn from(variant: ADDISR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` reader - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ADDIS_R = crate::BitReader<ADDISR>;
impl ADDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDISR {
        match self.bits {
            false => ADDISR::NotOngoing,
            true => ADDISR::InProgress,
        }
    }
    ///No ADDIS command ongoing
    #[inline(always)]
    pub fn is_not_ongoing(&self) -> bool {
        *self == ADDISR::NotOngoing
    }
    ///An ADDIS command is in progress
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == ADDISR::InProgress
    }
}
/**ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW {
    ///1: Disable the ADC
    Disable = 1,
}
impl From<ADDISW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDIS` writer - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG, ADDISW>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable the ADC
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADDISW::Disable)
    }
}
/**ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\[1:0\] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\[1:0\] = 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR {
    ///0: No ADC regular conversion is ongoing
    NotActive = 0,
    ///1: ADC is operating and eventually converting a regular channel
    Active = 1,
}
impl From<ADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` reader - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\[1:0\] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\[1:0\] = 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type ADSTART_R = crate::BitReader<ADSTARTR>;
impl ADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTARTR {
        match self.bits {
            false => ADSTARTR::NotActive,
            true => ADSTARTR::Active,
        }
    }
    ///No ADC regular conversion is ongoing
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR::NotActive
    }
    ///ADC is operating and eventually converting a regular channel
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR::Active
    }
}
/**ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\[1:0\] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\[1:0\] = 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW {
    ///1: Start regular conversions
    Start = 1,
}
impl From<ADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTART` writer - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\[1:0\] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\[1:0\] = 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG, ADSTARTW>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start regular conversions
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTARTW::Start)
    }
}
/**ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JADSTARTR {
    ///0: No ADC injected conversion is ongoing
    NotActive = 0,
    ///1: ADC is operating and eventually converting an injected channel
    Active = 1,
}
impl From<JADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: JADSTARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `JADSTART` reader - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type JADSTART_R = crate::BitReader<JADSTARTR>;
impl JADSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JADSTARTR {
        match self.bits {
            false => JADSTARTR::NotActive,
            true => JADSTARTR::Active,
        }
    }
    ///No ADC injected conversion is ongoing
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == JADSTARTR::NotActive
    }
    ///ADC is operating and eventually converting an injected channel
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == JADSTARTR::Active
    }
}
/**ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JADSTARTW {
    ///1: Start injected conversions
    Start = 1,
}
impl From<JADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: JADSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `JADSTART` writer - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
pub type JADSTART_W<'a, REG> = crate::BitWriter<'a, REG, JADSTARTW>;
impl<'a, REG> JADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start injected conversions
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(JADSTARTW::Start)
    }
}
/**ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR {
    ///0: No ADC stop regular conversion command ongoing
    NotStopping = 0,
    ///1: ADSTP command is in progress
    Stopping = 1,
}
impl From<ADSTPR> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` reader - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
pub type ADSTP_R = crate::BitReader<ADSTPR>;
impl ADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSTPR {
        match self.bits {
            false => ADSTPR::NotStopping,
            true => ADSTPR::Stopping,
        }
    }
    ///No ADC stop regular conversion command ongoing
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR::NotStopping
    }
    ///ADSTP command is in progress
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR::Stopping
    }
}
/**ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW {
    ///1: Stop regular conversions ongoing
    StopConversion = 1,
}
impl From<ADSTPW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADSTP` writer - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG, ADSTPW>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop regular conversions ongoing
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTPW::StopConversion)
    }
}
/**ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JADSTPR {
    ///0: No ADC stop injected conversion command ongoing
    NotStopped = 0,
    ///1: ADSTP command is in progress
    Stopped = 1,
}
impl From<JADSTPR> for bool {
    #[inline(always)]
    fn from(variant: JADSTPR) -> Self {
        variant as u8 != 0
    }
}
///Field `JADSTP` reader - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
pub type JADSTP_R = crate::BitReader<JADSTPR>;
impl JADSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JADSTPR {
        match self.bits {
            false => JADSTPR::NotStopped,
            true => JADSTPR::Stopped,
        }
    }
    ///No ADC stop injected conversion command ongoing
    #[inline(always)]
    pub fn is_not_stopped(&self) -> bool {
        *self == JADSTPR::NotStopped
    }
    ///ADSTP command is in progress
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == JADSTPR::Stopped
    }
}
/**ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JADSTPW {
    ///1: Stop injected conversions ongoing
    Stop = 1,
}
impl From<JADSTPW> for bool {
    #[inline(always)]
    fn from(variant: JADSTPW) -> Self {
        variant as u8 != 0
    }
}
///Field `JADSTP` writer - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
pub type JADSTP_W<'a, REG> = crate::BitWriter<'a, REG, JADSTPW>;
impl<'a, REG> JADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop injected conversions ongoing
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(JADSTPW::Stop)
    }
}
/**Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALLIN {
    ///0: Writing ADCAL launches a calibration without the linearity calibration
    Disabled = 0,
    ///1: Writing ADCAL launches a calibration with he linearity calibration
    Enabled = 1,
}
impl From<ADCALLIN> for bool {
    #[inline(always)]
    fn from(variant: ADCALLIN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCALLIN` reader - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADCALLIN_R = crate::BitReader<ADCALLIN>;
impl ADCALLIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALLIN {
        match self.bits {
            false => ADCALLIN::Disabled,
            true => ADCALLIN::Enabled,
        }
    }
    ///Writing ADCAL launches a calibration without the linearity calibration
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCALLIN::Disabled
    }
    ///Writing ADCAL launches a calibration with he linearity calibration
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCALLIN::Enabled
    }
}
///Field `ADCALLIN` writer - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADCALLIN_W<'a, REG> = crate::BitWriter<'a, REG, ADCALLIN>;
impl<'a, REG> ADCALLIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing ADCAL launches a calibration without the linearity calibration
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALLIN::Disabled)
    }
    ///Writing ADCAL launches a calibration with he linearity calibration
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALLIN::Enabled)
    }
}
/**Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\[31:0\] correspond to the location of CALINDEX\[3:0\] calibration factor data (see for details).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALINDEX {
    ///0: Offset calibration factor
    OffsetCalFactor = 0,
    ///1: Calibration factor 1
    CalFactor1 = 1,
    ///2: Calibration factor 2
    CalFactor2 = 2,
    ///3: Calibration factor 3
    CalFactor3 = 3,
    ///4: Calibration factor 4
    CalFactor4 = 4,
    ///5: Calibration factor 5
    CalFactor5 = 5,
    ///6: Calibration factor 6
    CalFactor6 = 6,
    ///7: Calibration factor 7 and (write access only) internal offset
    CalFactor7 = 7,
    ///8: Internal offset (read access only)
    InternalOffset = 8,
    ///9: Calibration mode selection
    CalibrationMode = 9,
}
impl From<CALINDEX> for u8 {
    #[inline(always)]
    fn from(variant: CALINDEX) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CALINDEX {
    type Ux = u8;
}
impl crate::IsEnum for CALINDEX {}
///Field `CALINDEX` reader - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\[31:0\] correspond to the location of CALINDEX\[3:0\] calibration factor data (see for details).
pub type CALINDEX_R = crate::FieldReader<CALINDEX>;
impl CALINDEX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALINDEX> {
        match self.bits {
            0 => Some(CALINDEX::OffsetCalFactor),
            1 => Some(CALINDEX::CalFactor1),
            2 => Some(CALINDEX::CalFactor2),
            3 => Some(CALINDEX::CalFactor3),
            4 => Some(CALINDEX::CalFactor4),
            5 => Some(CALINDEX::CalFactor5),
            6 => Some(CALINDEX::CalFactor6),
            7 => Some(CALINDEX::CalFactor7),
            8 => Some(CALINDEX::InternalOffset),
            9 => Some(CALINDEX::CalibrationMode),
            _ => None,
        }
    }
    ///Offset calibration factor
    #[inline(always)]
    pub fn is_offset_cal_factor(&self) -> bool {
        *self == CALINDEX::OffsetCalFactor
    }
    ///Calibration factor 1
    #[inline(always)]
    pub fn is_cal_factor1(&self) -> bool {
        *self == CALINDEX::CalFactor1
    }
    ///Calibration factor 2
    #[inline(always)]
    pub fn is_cal_factor2(&self) -> bool {
        *self == CALINDEX::CalFactor2
    }
    ///Calibration factor 3
    #[inline(always)]
    pub fn is_cal_factor3(&self) -> bool {
        *self == CALINDEX::CalFactor3
    }
    ///Calibration factor 4
    #[inline(always)]
    pub fn is_cal_factor4(&self) -> bool {
        *self == CALINDEX::CalFactor4
    }
    ///Calibration factor 5
    #[inline(always)]
    pub fn is_cal_factor5(&self) -> bool {
        *self == CALINDEX::CalFactor5
    }
    ///Calibration factor 6
    #[inline(always)]
    pub fn is_cal_factor6(&self) -> bool {
        *self == CALINDEX::CalFactor6
    }
    ///Calibration factor 7 and (write access only) internal offset
    #[inline(always)]
    pub fn is_cal_factor7(&self) -> bool {
        *self == CALINDEX::CalFactor7
    }
    ///Internal offset (read access only)
    #[inline(always)]
    pub fn is_internal_offset(&self) -> bool {
        *self == CALINDEX::InternalOffset
    }
    ///Calibration mode selection
    #[inline(always)]
    pub fn is_calibration_mode(&self) -> bool {
        *self == CALINDEX::CalibrationMode
    }
}
///Field `CALINDEX` writer - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\[31:0\] correspond to the location of CALINDEX\[3:0\] calibration factor data (see for details).
pub type CALINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CALINDEX>;
impl<'a, REG> CALINDEX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Offset calibration factor
    #[inline(always)]
    pub fn offset_cal_factor(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::OffsetCalFactor)
    }
    ///Calibration factor 1
    #[inline(always)]
    pub fn cal_factor1(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor1)
    }
    ///Calibration factor 2
    #[inline(always)]
    pub fn cal_factor2(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor2)
    }
    ///Calibration factor 3
    #[inline(always)]
    pub fn cal_factor3(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor3)
    }
    ///Calibration factor 4
    #[inline(always)]
    pub fn cal_factor4(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor4)
    }
    ///Calibration factor 5
    #[inline(always)]
    pub fn cal_factor5(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor5)
    }
    ///Calibration factor 6
    #[inline(always)]
    pub fn cal_factor6(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor6)
    }
    ///Calibration factor 7 and (write access only) internal offset
    #[inline(always)]
    pub fn cal_factor7(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalFactor7)
    }
    ///Internal offset (read access only)
    #[inline(always)]
    pub fn internal_offset(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::InternalOffset)
    }
    ///Calibration mode selection
    #[inline(always)]
    pub fn calibration_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CALINDEX::CalibrationMode)
    }
}
/**ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN {
    ///0: ADC voltage regulator disabled
    Disabled = 0,
    ///1: ADC voltage regulator enabled
    Enabled = 1,
}
impl From<ADVREGEN> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADVREGEN` reader - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_R = crate::BitReader<ADVREGEN>;
impl ADVREGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADVREGEN {
        match self.bits {
            false => ADVREGEN::Disabled,
            true => ADVREGEN::Enabled,
        }
    }
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN::Disabled
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN::Enabled
    }
}
///Field `ADVREGEN` writer - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type ADVREGEN_W<'a, REG> = crate::BitWriter<'a, REG, ADVREGEN>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC voltage regulator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Disabled)
    }
    ///ADC voltage regulator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Enabled)
    }
}
/**Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPWD {
    ///0: ADC not in deep-power down
    Disabled = 0,
    ///1: ADC in deep-power down
    Enabled = 1,
}
impl From<DEEPPWD> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD) -> Self {
        variant as u8 != 0
    }
}
///Field `DEEPPWD` reader - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DEEPPWD_R = crate::BitReader<DEEPPWD>;
impl DEEPPWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEEPPWD {
        match self.bits {
            false => DEEPPWD::Disabled,
            true => DEEPPWD::Enabled,
        }
    }
    ///ADC not in deep-power down
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEEPPWD::Disabled
    }
    ///ADC in deep-power down
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEEPPWD::Enabled
    }
}
///Field `DEEPPWD` writer - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type DEEPPWD_W<'a, REG> = crate::BitWriter<'a, REG, DEEPPWD>;
impl<'a, REG> DEEPPWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC not in deep-power down
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::Disabled)
    }
    ///ADC in deep-power down
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPPWD::Enabled)
    }
}
/**ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR {
    ///0: Calibration complete
    NotCalibrating = 0,
    ///1: Calibration in progress
    Calibrating = 1,
}
impl From<ADCALR> for bool {
    #[inline(always)]
    fn from(variant: ADCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` reader - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.
pub type ADCAL_R = crate::BitReader<ADCALR>;
impl ADCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCALR {
        match self.bits {
            false => ADCALR::NotCalibrating,
            true => ADCALR::Calibrating,
        }
    }
    ///Calibration complete
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        *self == ADCALR::NotCalibrating
    }
    ///Calibration in progress
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        *self == ADCALR::Calibrating
    }
}
/**ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW {
    ///1: Calibrate the ADC
    StartCalibration = 1,
}
impl From<ADCALW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCAL` writer - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCALW>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibrate the ADC
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW::StartCalibration)
    }
}
impl R {
    ///Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\[1:0\] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\[1:0\] = 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 24:27 - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\[31:0\] correspond to the location of CALINDEX\[3:0\] calibration factor data (see for details).
    #[inline(always)]
    pub fn calindex(&self) -> CALINDEX_R {
        CALINDEX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("aden", &self.aden())
            .field("addis", &self.addis())
            .field("adstart", &self.adstart())
            .field("jadstart", &self.jadstart())
            .field("adstp", &self.adstp())
            .field("jadstp", &self.jadstp())
            .field("adcallin", &self.adcallin())
            .field("calindex", &self.calindex())
            .field("advregen", &self.advregen())
            .field("deeppwd", &self.deeppwd())
            .field("adcal", &self.adcal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator)
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W<CRrs> {
        ADEN_W::new(self, 0)
    }
    ///Bit 1 - ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W<CRrs> {
        ADDIS_W::new(self, 1)
    }
    ///Bit 2 - ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected (EXTEN\[1:0\] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is selected (EXTEN\[1:0\] = 0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W<CRrs> {
        ADSTART_W::new(self, 2)
    }
    ///Bit 3 - ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN\[1:0\], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared)
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W<CRrs> {
        JADSTART_W::new(self, 3)
    }
    ///Bit 4 - ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W<CRrs> {
        ADSTP_W::new(self, 4)
    }
    ///Bit 5 - ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP)
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W<CRrs> {
        JADSTP_W::new(self, 5)
    }
    ///Bit 16 - Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn adcallin(&mut self) -> ADCALLIN_W<CRrs> {
        ADCALLIN_W::new(self, 16)
    }
    ///Bits 24:27 - Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: ADC_CALFACT2\[31:0\] correspond to the location of CALINDEX\[3:0\] calibration factor data (see for details).
    #[inline(always)]
    pub fn calindex(&mut self) -> CALINDEX_W<CRrs> {
        CALINDEX_W::new(self, 24)
    }
    ///Bit 28 - ADC voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W<CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    ///Bit 29 - Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<CRrs> {
        DEEPPWD_W::new(self, 29)
    }
    ///Bit 31 - ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W<CRrs> {
        ADCAL_W::new(self, 31)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x2000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
