///Register `CR4` reader
pub type R = crate::R<CR4rs>;
///Register `CR4` writer
pub type W = crate::W<CR4rs>;
/**

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1PDS4 {
    ///0: SRAM1 page x content retained in Stop modes
    Disabled = 0,
    ///1: SRAM1 page x content lost in Stop modes
    Enabled = 1,
}
impl From<SRAM1PDS4> for bool {
    #[inline(always)]
    fn from(variant: SRAM1PDS4) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM1PDS4` reader -
pub type SRAM1PDS4_R = crate::BitReader<SRAM1PDS4>;
impl SRAM1PDS4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1PDS4 {
        match self.bits {
            false => SRAM1PDS4::Disabled,
            true => SRAM1PDS4::Enabled,
        }
    }
    ///SRAM1 page x content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM1PDS4::Disabled
    }
    ///SRAM1 page x content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM1PDS4::Enabled
    }
}
///Field `SRAM1PDS4` writer -
pub type SRAM1PDS4_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1PDS4>;
impl<'a, REG> SRAM1PDS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM1 page x content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1PDS4::Disabled)
    }
    ///SRAM1 page x content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1PDS4::Enabled)
    }
}
///Field `SRAM1PDS5` reader -
pub use SRAM1PDS4_R as SRAM1PDS5_R;
///Field `SRAM1PDS6` reader -
pub use SRAM1PDS4_R as SRAM1PDS6_R;
///Field `SRAM1PDS7` reader -
pub use SRAM1PDS4_R as SRAM1PDS7_R;
///Field `SRAM1PDS8` reader -
pub use SRAM1PDS4_R as SRAM1PDS8_R;
///Field `SRAM1PDS9` reader -
pub use SRAM1PDS4_R as SRAM1PDS9_R;
///Field `SRAM1PDS10` reader -
pub use SRAM1PDS4_R as SRAM1PDS10_R;
///Field `SRAM1PDS11` reader -
pub use SRAM1PDS4_R as SRAM1PDS11_R;
///Field `SRAM1PDS12` reader -
pub use SRAM1PDS4_R as SRAM1PDS12_R;
///Field `SRAM1PDS5` writer -
pub use SRAM1PDS4_W as SRAM1PDS5_W;
///Field `SRAM1PDS6` writer -
pub use SRAM1PDS4_W as SRAM1PDS6_W;
///Field `SRAM1PDS7` writer -
pub use SRAM1PDS4_W as SRAM1PDS7_W;
///Field `SRAM1PDS8` writer -
pub use SRAM1PDS4_W as SRAM1PDS8_W;
///Field `SRAM1PDS9` writer -
pub use SRAM1PDS4_W as SRAM1PDS9_W;
///Field `SRAM1PDS10` writer -
pub use SRAM1PDS4_W as SRAM1PDS10_W;
///Field `SRAM1PDS11` writer -
pub use SRAM1PDS4_W as SRAM1PDS11_W;
///Field `SRAM1PDS12` writer -
pub use SRAM1PDS4_W as SRAM1PDS12_W;
/**

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM3PDS9 {
    ///0: SRAM3 page x content retained in Stop modes
    Disabled = 0,
    ///1: SRAM3 page x content lost in Stop modes
    Enabled = 1,
}
impl From<SRAM3PDS9> for bool {
    #[inline(always)]
    fn from(variant: SRAM3PDS9) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM3PDS9` reader -
pub type SRAM3PDS9_R = crate::BitReader<SRAM3PDS9>;
impl SRAM3PDS9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM3PDS9 {
        match self.bits {
            false => SRAM3PDS9::Disabled,
            true => SRAM3PDS9::Enabled,
        }
    }
    ///SRAM3 page x content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM3PDS9::Disabled
    }
    ///SRAM3 page x content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM3PDS9::Enabled
    }
}
///Field `SRAM3PDS9` writer -
pub type SRAM3PDS9_W<'a, REG> = crate::BitWriter<'a, REG, SRAM3PDS9>;
impl<'a, REG> SRAM3PDS9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM3 page x content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM3PDS9::Disabled)
    }
    ///SRAM3 page x content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM3PDS9::Enabled)
    }
}
///Field `SRAM3PDS10` reader -
pub use SRAM3PDS9_R as SRAM3PDS10_R;
///Field `SRAM3PDS11` reader -
pub use SRAM3PDS9_R as SRAM3PDS11_R;
///Field `SRAM3PDS12` reader -
pub use SRAM3PDS9_R as SRAM3PDS12_R;
///Field `SRAM3PDS13` reader -
pub use SRAM3PDS9_R as SRAM3PDS13_R;
///Field `SRAM3PDS10` writer -
pub use SRAM3PDS9_W as SRAM3PDS10_W;
///Field `SRAM3PDS11` writer -
pub use SRAM3PDS9_W as SRAM3PDS11_W;
///Field `SRAM3PDS12` writer -
pub use SRAM3PDS9_W as SRAM3PDS12_W;
///Field `SRAM3PDS13` writer -
pub use SRAM3PDS9_W as SRAM3PDS13_W;
/**

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM5PDS1 {
    ///0: SRAM5 page x content retained in Stop modes
    Disabled = 0,
    ///1: SRAM5 page x content lost in Stop modes
    Enabled = 1,
}
impl From<SRAM5PDS1> for bool {
    #[inline(always)]
    fn from(variant: SRAM5PDS1) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM5PDS1` reader -
pub type SRAM5PDS1_R = crate::BitReader<SRAM5PDS1>;
impl SRAM5PDS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM5PDS1 {
        match self.bits {
            false => SRAM5PDS1::Disabled,
            true => SRAM5PDS1::Enabled,
        }
    }
    ///SRAM5 page x content retained in Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM5PDS1::Disabled
    }
    ///SRAM5 page x content lost in Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM5PDS1::Enabled
    }
}
///Field `SRAM5PDS1` writer -
pub type SRAM5PDS1_W<'a, REG> = crate::BitWriter<'a, REG, SRAM5PDS1>;
impl<'a, REG> SRAM5PDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM5 page x content retained in Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM5PDS1::Disabled)
    }
    ///SRAM5 page x content lost in Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM5PDS1::Enabled)
    }
}
///Field `SRAM5PDS2` reader -
pub use SRAM5PDS1_R as SRAM5PDS2_R;
///Field `SRAM5PDS3` reader -
pub use SRAM5PDS1_R as SRAM5PDS3_R;
///Field `SRAM5PDS4` reader -
pub use SRAM5PDS1_R as SRAM5PDS4_R;
///Field `SRAM5PDS5` reader -
pub use SRAM5PDS1_R as SRAM5PDS5_R;
///Field `SRAM5PDS6` reader -
pub use SRAM5PDS1_R as SRAM5PDS6_R;
///Field `SRAM5PDS7` reader -
pub use SRAM5PDS1_R as SRAM5PDS7_R;
///Field `SRAM5PDS8` reader -
pub use SRAM5PDS1_R as SRAM5PDS8_R;
///Field `SRAM5PDS9` reader -
pub use SRAM5PDS1_R as SRAM5PDS9_R;
///Field `SRAM5PDS10` reader -
pub use SRAM5PDS1_R as SRAM5PDS10_R;
///Field `SRAM5PDS11` reader -
pub use SRAM5PDS1_R as SRAM5PDS11_R;
///Field `SRAM5PDS12` reader -
pub use SRAM5PDS1_R as SRAM5PDS12_R;
///Field `SRAM5PDS13` reader -
pub use SRAM5PDS1_R as SRAM5PDS13_R;
///Field `SRAM5PDS2` writer -
pub use SRAM5PDS1_W as SRAM5PDS2_W;
///Field `SRAM5PDS3` writer -
pub use SRAM5PDS1_W as SRAM5PDS3_W;
///Field `SRAM5PDS4` writer -
pub use SRAM5PDS1_W as SRAM5PDS4_W;
///Field `SRAM5PDS5` writer -
pub use SRAM5PDS1_W as SRAM5PDS5_W;
///Field `SRAM5PDS6` writer -
pub use SRAM5PDS1_W as SRAM5PDS6_W;
///Field `SRAM5PDS7` writer -
pub use SRAM5PDS1_W as SRAM5PDS7_W;
///Field `SRAM5PDS8` writer -
pub use SRAM5PDS1_W as SRAM5PDS8_W;
///Field `SRAM5PDS9` writer -
pub use SRAM5PDS1_W as SRAM5PDS9_W;
///Field `SRAM5PDS10` writer -
pub use SRAM5PDS1_W as SRAM5PDS10_W;
///Field `SRAM5PDS11` writer -
pub use SRAM5PDS1_W as SRAM5PDS11_W;
///Field `SRAM5PDS12` writer -
pub use SRAM5PDS1_W as SRAM5PDS12_W;
///Field `SRAM5PDS13` writer -
pub use SRAM5PDS1_W as SRAM5PDS13_W;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn sram1pds4(&self) -> SRAM1PDS4_R {
        SRAM1PDS4_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn sram1pds5(&self) -> SRAM1PDS5_R {
        SRAM1PDS5_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn sram1pds6(&self) -> SRAM1PDS6_R {
        SRAM1PDS6_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn sram1pds7(&self) -> SRAM1PDS7_R {
        SRAM1PDS7_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn sram1pds8(&self) -> SRAM1PDS8_R {
        SRAM1PDS8_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn sram1pds9(&self) -> SRAM1PDS9_R {
        SRAM1PDS9_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn sram1pds10(&self) -> SRAM1PDS10_R {
        SRAM1PDS10_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn sram1pds11(&self) -> SRAM1PDS11_R {
        SRAM1PDS11_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn sram1pds12(&self) -> SRAM1PDS12_R {
        SRAM1PDS12_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn sram3pds9(&self) -> SRAM3PDS9_R {
        SRAM3PDS9_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn sram3pds10(&self) -> SRAM3PDS10_R {
        SRAM3PDS10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn sram3pds11(&self) -> SRAM3PDS11_R {
        SRAM3PDS11_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn sram3pds12(&self) -> SRAM3PDS12_R {
        SRAM3PDS12_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn sram3pds13(&self) -> SRAM3PDS13_R {
        SRAM3PDS13_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn sram5pds1(&self) -> SRAM5PDS1_R {
        SRAM5PDS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn sram5pds2(&self) -> SRAM5PDS2_R {
        SRAM5PDS2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn sram5pds3(&self) -> SRAM5PDS3_R {
        SRAM5PDS3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn sram5pds4(&self) -> SRAM5PDS4_R {
        SRAM5PDS4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn sram5pds5(&self) -> SRAM5PDS5_R {
        SRAM5PDS5_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn sram5pds6(&self) -> SRAM5PDS6_R {
        SRAM5PDS6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn sram5pds7(&self) -> SRAM5PDS7_R {
        SRAM5PDS7_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23
    #[inline(always)]
    pub fn sram5pds8(&self) -> SRAM5PDS8_R {
        SRAM5PDS8_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn sram5pds9(&self) -> SRAM5PDS9_R {
        SRAM5PDS9_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25
    #[inline(always)]
    pub fn sram5pds10(&self) -> SRAM5PDS10_R {
        SRAM5PDS10_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26
    #[inline(always)]
    pub fn sram5pds11(&self) -> SRAM5PDS11_R {
        SRAM5PDS11_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27
    #[inline(always)]
    pub fn sram5pds12(&self) -> SRAM5PDS12_R {
        SRAM5PDS12_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28
    #[inline(always)]
    pub fn sram5pds13(&self) -> SRAM5PDS13_R {
        SRAM5PDS13_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("sram1pds4", &self.sram1pds4())
            .field("sram1pds5", &self.sram1pds5())
            .field("sram1pds6", &self.sram1pds6())
            .field("sram1pds7", &self.sram1pds7())
            .field("sram1pds8", &self.sram1pds8())
            .field("sram1pds9", &self.sram1pds9())
            .field("sram1pds10", &self.sram1pds10())
            .field("sram1pds11", &self.sram1pds11())
            .field("sram1pds12", &self.sram1pds12())
            .field("sram3pds9", &self.sram3pds9())
            .field("sram3pds10", &self.sram3pds10())
            .field("sram3pds11", &self.sram3pds11())
            .field("sram3pds12", &self.sram3pds12())
            .field("sram3pds13", &self.sram3pds13())
            .field("sram5pds1", &self.sram5pds1())
            .field("sram5pds2", &self.sram5pds2())
            .field("sram5pds3", &self.sram5pds3())
            .field("sram5pds4", &self.sram5pds4())
            .field("sram5pds5", &self.sram5pds5())
            .field("sram5pds6", &self.sram5pds6())
            .field("sram5pds7", &self.sram5pds7())
            .field("sram5pds8", &self.sram5pds8())
            .field("sram5pds9", &self.sram5pds9())
            .field("sram5pds10", &self.sram5pds10())
            .field("sram5pds11", &self.sram5pds11())
            .field("sram5pds12", &self.sram5pds12())
            .field("sram5pds13", &self.sram5pds13())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn sram1pds4(&mut self) -> SRAM1PDS4_W<CR4rs> {
        SRAM1PDS4_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn sram1pds5(&mut self) -> SRAM1PDS5_W<CR4rs> {
        SRAM1PDS5_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn sram1pds6(&mut self) -> SRAM1PDS6_W<CR4rs> {
        SRAM1PDS6_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn sram1pds7(&mut self) -> SRAM1PDS7_W<CR4rs> {
        SRAM1PDS7_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    pub fn sram1pds8(&mut self) -> SRAM1PDS8_W<CR4rs> {
        SRAM1PDS8_W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn sram1pds9(&mut self) -> SRAM1PDS9_W<CR4rs> {
        SRAM1PDS9_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn sram1pds10(&mut self) -> SRAM1PDS10_W<CR4rs> {
        SRAM1PDS10_W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn sram1pds11(&mut self) -> SRAM1PDS11_W<CR4rs> {
        SRAM1PDS11_W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn sram1pds12(&mut self) -> SRAM1PDS12_W<CR4rs> {
        SRAM1PDS12_W::new(self, 8)
    }
    ///Bit 10
    #[inline(always)]
    pub fn sram3pds9(&mut self) -> SRAM3PDS9_W<CR4rs> {
        SRAM3PDS9_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn sram3pds10(&mut self) -> SRAM3PDS10_W<CR4rs> {
        SRAM3PDS10_W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    pub fn sram3pds11(&mut self) -> SRAM3PDS11_W<CR4rs> {
        SRAM3PDS11_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    pub fn sram3pds12(&mut self) -> SRAM3PDS12_W<CR4rs> {
        SRAM3PDS12_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    pub fn sram3pds13(&mut self) -> SRAM3PDS13_W<CR4rs> {
        SRAM3PDS13_W::new(self, 14)
    }
    ///Bit 16
    #[inline(always)]
    pub fn sram5pds1(&mut self) -> SRAM5PDS1_W<CR4rs> {
        SRAM5PDS1_W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn sram5pds2(&mut self) -> SRAM5PDS2_W<CR4rs> {
        SRAM5PDS2_W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn sram5pds3(&mut self) -> SRAM5PDS3_W<CR4rs> {
        SRAM5PDS3_W::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn sram5pds4(&mut self) -> SRAM5PDS4_W<CR4rs> {
        SRAM5PDS4_W::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn sram5pds5(&mut self) -> SRAM5PDS5_W<CR4rs> {
        SRAM5PDS5_W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn sram5pds6(&mut self) -> SRAM5PDS6_W<CR4rs> {
        SRAM5PDS6_W::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    pub fn sram5pds7(&mut self) -> SRAM5PDS7_W<CR4rs> {
        SRAM5PDS7_W::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    pub fn sram5pds8(&mut self) -> SRAM5PDS8_W<CR4rs> {
        SRAM5PDS8_W::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    pub fn sram5pds9(&mut self) -> SRAM5PDS9_W<CR4rs> {
        SRAM5PDS9_W::new(self, 24)
    }
    ///Bit 25
    #[inline(always)]
    pub fn sram5pds10(&mut self) -> SRAM5PDS10_W<CR4rs> {
        SRAM5PDS10_W::new(self, 25)
    }
    ///Bit 26
    #[inline(always)]
    pub fn sram5pds11(&mut self) -> SRAM5PDS11_W<CR4rs> {
        SRAM5PDS11_W::new(self, 26)
    }
    ///Bit 27
    #[inline(always)]
    pub fn sram5pds12(&mut self) -> SRAM5PDS12_W<CR4rs> {
        SRAM5PDS12_W::new(self, 27)
    }
    ///Bit 28
    #[inline(always)]
    pub fn sram5pds13(&mut self) -> SRAM5PDS13_W<CR4rs> {
        SRAM5PDS13_W::new(self, 28)
    }
}
/**PWR control register 4

You can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#PWR:CR4)*/
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
///`read()` method returns [`cr4::R`](R) reader structure
impl crate::Readable for CR4rs {}
///`write(|w| ..)` method takes [`cr4::W`](W) writer structure
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR4 to value 0
impl crate::Resettable for CR4rs {}
