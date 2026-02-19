///Register `OFR%s` reader
pub type R = crate::R<OFRrs>;
///Register `OFR%s` writer
pub type W = crate::W<OFRrs>;
///Field `OFFSET` reader - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\] bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\] bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[25:0\] that is subtracted when converting channel 4.
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\] bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\] bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[25:0\] that is subtracted when converting channel 4.
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
/**offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSOFF {
    ///0: Negative offset
    Negative = 0,
    ///1: Positive offset
    Positive = 1,
}
impl From<POSOFF> for bool {
    #[inline(always)]
    fn from(variant: POSOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `POSOFF` reader - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type POSOFF_R = crate::BitReader<POSOFF>;
impl POSOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POSOFF {
        match self.bits {
            false => POSOFF::Negative,
            true => POSOFF::Positive,
        }
    }
    ///Negative offset
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == POSOFF::Negative
    }
    ///Positive offset
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == POSOFF::Positive
    }
}
///Field `POSOFF` writer - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type POSOFF_W<'a, REG> = crate::BitWriter<'a, REG, POSOFF>;
impl<'a, REG> POSOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Negative offset
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(POSOFF::Negative)
    }
    ///Positive offset
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(POSOFF::Positive)
    }
}
/**Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USAT {
    ///0: Offset is subtracted maintaining data integrity and keeping converted data size
    Disabled = 0,
    ///1: Offset is subtracted and result is saturated to maintain converted data size
    Enabled = 1,
}
impl From<USAT> for bool {
    #[inline(always)]
    fn from(variant: USAT) -> Self {
        variant as u8 != 0
    }
}
///Field `USAT` reader - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type USAT_R = crate::BitReader<USAT>;
impl USAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USAT {
        match self.bits {
            false => USAT::Disabled,
            true => USAT::Enabled,
        }
    }
    ///Offset is subtracted maintaining data integrity and keeping converted data size
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USAT::Disabled
    }
    ///Offset is subtracted and result is saturated to maintain converted data size
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USAT::Enabled
    }
}
///Field `USAT` writer - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type USAT_W<'a, REG> = crate::BitWriter<'a, REG, USAT>;
impl<'a, REG> USAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset is subtracted maintaining data integrity and keeping converted data size
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USAT::Disabled)
    }
    ///Offset is subtracted and result is saturated to maintain converted data size
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USAT::Enabled)
    }
}
/**Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSAT {
    ///0: Offset is subtracted maintaining data integrity and extending converted data size (9-bit and 15-bit signed format)
    Disabled = 0,
    ///1: Offset is subtracted and result is saturated to maintain converted data size
    Enabled = 1,
}
impl From<SSAT> for bool {
    #[inline(always)]
    fn from(variant: SSAT) -> Self {
        variant as u8 != 0
    }
}
///Field `SSAT` reader - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SSAT_R = crate::BitReader<SSAT>;
impl SSAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSAT {
        match self.bits {
            false => SSAT::Disabled,
            true => SSAT::Enabled,
        }
    }
    ///Offset is subtracted maintaining data integrity and extending converted data size (9-bit and 15-bit signed format)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSAT::Disabled
    }
    ///Offset is subtracted and result is saturated to maintain converted data size
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSAT::Enabled
    }
}
///Field `SSAT` writer - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SSAT_W<'a, REG> = crate::BitWriter<'a, REG, SSAT>;
impl<'a, REG> SSAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset is subtracted maintaining data integrity and extending converted data size (9-bit and 15-bit signed format)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSAT::Disabled)
    }
    ///Offset is subtracted and result is saturated to maintain converted data size
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSAT::Enabled)
    }
}
///Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\] bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\] bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\] bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\] bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[25:0\] that is subtracted when converting channel 4.
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\] bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR")
            .field("offset", &self.offset())
            .field("posoff", &self.posoff())
            .field("usat", &self.usat())
            .field("ssat", &self.ssat())
            .field("offset_ch", &self.offset_ch())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\] bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\] bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\] bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[25:0\] that is subtracted when converting channel 4.
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<OFRrs> {
        OFFSET_W::new(self, 0)
    }
    ///Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn posoff(&mut self) -> POSOFF_W<OFRrs> {
        POSOFF_W::new(self, 24)
    }
    ///Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn usat(&mut self) -> USAT_W<OFRrs> {
        USAT_W::new(self, 25)
    }
    ///Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ssat(&mut self) -> SSAT_W<OFRrs> {
        SSAT_W::new(self, 26)
    }
    ///Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\] bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.
    #[inline(always)]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<OFRrs> {
        OFFSET_CH_W::new(self, 27)
    }
}
/**ADC offset register

You can [`read`](crate::Reg::read) this register and get [`ofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:OFR[1])*/
pub struct OFRrs;
impl crate::RegisterSpec for OFRrs {
    type Ux = u32;
}
///`read()` method returns [`ofr::R`](R) reader structure
impl crate::Readable for OFRrs {}
///`write(|w| ..)` method takes [`ofr::W`](W) writer structure
impl crate::Writable for OFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR%s to value 0
impl crate::Resettable for OFRrs {}
