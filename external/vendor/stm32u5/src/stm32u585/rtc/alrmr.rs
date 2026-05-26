///Register `ALRM%sR` reader
pub type R = crate::R<ALRMRrs>;
///Register `ALRM%sR` writer
pub type W = crate::W<ALRMRrs>;
///Field `SU` reader - Second units in BCD format
pub type SU_R = crate::FieldReader;
///Field `SU` writer - Second units in BCD format
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `ST` reader - Second tens in BCD format
pub type ST_R = crate::FieldReader;
///Field `ST` writer - Second tens in BCD format
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Alarm seconds mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK1 {
    ///0: Alarm set if the date/day match
    Mask = 0,
    ///1: Date/day don’t care in Alarm comparison
    NotMask = 1,
}
impl From<MSK1> for bool {
    #[inline(always)]
    fn from(variant: MSK1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSK1` reader - Alarm seconds mask
pub type MSK1_R = crate::BitReader<MSK1>;
impl MSK1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSK1 {
        match self.bits {
            false => MSK1::Mask,
            true => MSK1::NotMask,
        }
    }
    ///Alarm set if the date/day match
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MSK1::Mask
    }
    ///Date/day don’t care in Alarm comparison
    #[inline(always)]
    pub fn is_not_mask(&self) -> bool {
        *self == MSK1::NotMask
    }
}
///Field `MSK1` writer - Alarm seconds mask
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG, MSK1>;
impl<'a, REG> MSK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm set if the date/day match
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1::Mask)
    }
    ///Date/day don’t care in Alarm comparison
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1::NotMask)
    }
}
///Field `MNU` reader - Minute units in BCD format
pub type MNU_R = crate::FieldReader;
///Field `MNU` writer - Minute units in BCD format
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `MNT` reader - Minute tens in BCD format
pub type MNT_R = crate::FieldReader;
///Field `MNT` writer - Minute tens in BCD format
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Field `MSK2` reader - Alarm minutes mask
pub use MSK1_R as MSK2_R;
///Field `MSK2` writer - Alarm minutes mask
pub use MSK1_W as MSK2_W;
///Field `HU` reader - Hour units in BCD format
pub type HU_R = crate::FieldReader;
///Field `HU` writer - Hour units in BCD format
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `HT` reader - Hour tens in BCD format
pub type HT_R = crate::FieldReader;
///Field `HT` writer - Hour tens in BCD format
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
/**AM/PM notation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM {
    ///0: AM or 24-hour format
    Am = 0,
    ///1: PM
    Pm = 1,
}
impl From<PM> for bool {
    #[inline(always)]
    fn from(variant: PM) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - AM/PM notation
pub type PM_R = crate::BitReader<PM>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM {
        match self.bits {
            false => PM::Am,
            true => PM::Pm,
        }
    }
    ///AM or 24-hour format
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == PM::Am
    }
    ///PM
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == PM::Pm
    }
}
///Field `PM` writer - AM/PM notation
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AM or 24-hour format
    #[inline(always)]
    pub fn am(self) -> &'a mut crate::W<REG> {
        self.variant(PM::Am)
    }
    ///PM
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> {
        self.variant(PM::Pm)
    }
}
///Field `MSK3` reader - Alarm hours mask
pub use MSK1_R as MSK3_R;
///Field `MSK3` writer - Alarm hours mask
pub use MSK1_W as MSK3_W;
///Field `DU` reader - Date units or day in BCD format
pub type DU_R = crate::FieldReader;
///Field `DU` writer - Date units or day in BCD format
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
/**Week day selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDSEL {
    ///0: DU\[3:0\] represents the date units
    DateUnits = 0,
    ///1: DU\[3:0\] represents the week day. DT\[1:0\] is don’t care.
    WeekDay = 1,
}
impl From<WDSEL> for bool {
    #[inline(always)]
    fn from(variant: WDSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `WDSEL` reader - Week day selection
pub type WDSEL_R = crate::BitReader<WDSEL>;
impl WDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDSEL {
        match self.bits {
            false => WDSEL::DateUnits,
            true => WDSEL::WeekDay,
        }
    }
    ///DU\[3:0\] represents the date units
    #[inline(always)]
    pub fn is_date_units(&self) -> bool {
        *self == WDSEL::DateUnits
    }
    ///DU\[3:0\] represents the week day. DT\[1:0\] is don’t care.
    #[inline(always)]
    pub fn is_week_day(&self) -> bool {
        *self == WDSEL::WeekDay
    }
}
///Field `WDSEL` writer - Week day selection
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG, WDSEL>;
impl<'a, REG> WDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DU\[3:0\] represents the date units
    #[inline(always)]
    pub fn date_units(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL::DateUnits)
    }
    ///DU\[3:0\] represents the week day. DT\[1:0\] is don’t care.
    #[inline(always)]
    pub fn week_day(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL::WeekDay)
    }
}
///Field `MSK4` reader - Alarm date mask
pub use MSK1_R as MSK4_R;
///Field `MSK4` writer - Alarm date mask
pub use MSK1_W as MSK4_W;
impl R {
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Alarm seconds mask
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Alarm minutes mask
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Alarm hours mask
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Alarm date mask
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMR")
            .field("msk1", &self.msk1())
            .field("msk4", &self.msk4())
            .field("wdsel", &self.wdsel())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .field("msk3", &self.msk3())
            .field("pm", &self.pm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("msk2", &self.msk2())
            .field("mnt", &self.mnt())
            .field("mnu", &self.mnu())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<ALRMRrs> {
        SU_W::new(self, 0)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<ALRMRrs> {
        ST_W::new(self, 4)
    }
    ///Bit 7 - Alarm seconds mask
    #[inline(always)]
    pub fn msk1(&mut self) -> MSK1_W<ALRMRrs> {
        MSK1_W::new(self, 7)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<ALRMRrs> {
        MNU_W::new(self, 8)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<ALRMRrs> {
        MNT_W::new(self, 12)
    }
    ///Bit 15 - Alarm minutes mask
    #[inline(always)]
    pub fn msk2(&mut self) -> MSK2_W<ALRMRrs> {
        MSK2_W::new(self, 15)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<ALRMRrs> {
        HU_W::new(self, 16)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<ALRMRrs> {
        HT_W::new(self, 20)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<ALRMRrs> {
        PM_W::new(self, 22)
    }
    ///Bit 23 - Alarm hours mask
    #[inline(always)]
    pub fn msk3(&mut self) -> MSK3_W<ALRMRrs> {
        MSK3_W::new(self, 23)
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<ALRMRrs> {
        DU_W::new(self, 24)
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<ALRMRrs> {
        DT_W::new(self, 28)
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&mut self) -> WDSEL_W<ALRMRrs> {
        WDSEL_W::new(self, 30)
    }
    ///Bit 31 - Alarm date mask
    #[inline(always)]
    pub fn msk4(&mut self) -> MSK4_W<ALRMRrs> {
        MSK4_W::new(self, 31)
    }
}
/**Alarm %s register

You can [`read`](crate::Reg::read) this register and get [`alrmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#RTC:ALRM[A]R)*/
pub struct ALRMRrs;
impl crate::RegisterSpec for ALRMRrs {
    type Ux = u32;
}
///`read()` method returns [`alrmr::R`](R) reader structure
impl crate::Readable for ALRMRrs {}
///`write(|w| ..)` method takes [`alrmr::W`](W) writer structure
impl crate::Writable for ALRMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRM%sR to value 0
impl crate::Resettable for ALRMRrs {}
