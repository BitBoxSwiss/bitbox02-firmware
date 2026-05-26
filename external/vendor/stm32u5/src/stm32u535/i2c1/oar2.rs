///Register `OAR2` reader
pub type R = crate::R<OAR2rs>;
///Register `OAR2` writer
pub type W = crate::W<OAR2rs>;
///Field `OA2` reader - Interface address
pub type OA2_R = crate::FieldReader;
///Field `OA2` writer - Interface address
pub type OA2_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Own Address 2 masks

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OA2MSK {
    ///0: No mask
    NoMask = 0,
    ///1: OA2\[1\] is masked and don’t care. Only OA2\[7:2\] are compared
    Mask1 = 1,
    ///2: OA2\[2:1\] are masked and don’t care. Only OA2\[7:3\] are compared
    Mask2 = 2,
    ///3: OA2\[3:1\] are masked and don’t care. Only OA2\[7:4\] are compared
    Mask3 = 3,
    ///4: OA2\[4:1\] are masked and don’t care. Only OA2\[7:5\] are compared
    Mask4 = 4,
    ///5: OA2\[5:1\] are masked and don’t care. Only OA2\[7:6\] are compared
    Mask5 = 5,
    ///6: OA2\[6:1\] are masked and don’t care. Only OA2\[7\] is compared.
    Mask6 = 6,
    ///7: OA2\[7:1\] are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged
    Mask7 = 7,
}
impl From<OA2MSK> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OA2MSK {
    type Ux = u8;
}
impl crate::IsEnum for OA2MSK {}
///Field `OA2MSK` reader - Own Address 2 masks
pub type OA2MSK_R = crate::FieldReader<OA2MSK>;
impl OA2MSK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OA2MSK {
        match self.bits {
            0 => OA2MSK::NoMask,
            1 => OA2MSK::Mask1,
            2 => OA2MSK::Mask2,
            3 => OA2MSK::Mask3,
            4 => OA2MSK::Mask4,
            5 => OA2MSK::Mask5,
            6 => OA2MSK::Mask6,
            7 => OA2MSK::Mask7,
            _ => unreachable!(),
        }
    }
    ///No mask
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == OA2MSK::NoMask
    }
    ///OA2\[1\] is masked and don’t care. Only OA2\[7:2\] are compared
    #[inline(always)]
    pub fn is_mask1(&self) -> bool {
        *self == OA2MSK::Mask1
    }
    ///OA2\[2:1\] are masked and don’t care. Only OA2\[7:3\] are compared
    #[inline(always)]
    pub fn is_mask2(&self) -> bool {
        *self == OA2MSK::Mask2
    }
    ///OA2\[3:1\] are masked and don’t care. Only OA2\[7:4\] are compared
    #[inline(always)]
    pub fn is_mask3(&self) -> bool {
        *self == OA2MSK::Mask3
    }
    ///OA2\[4:1\] are masked and don’t care. Only OA2\[7:5\] are compared
    #[inline(always)]
    pub fn is_mask4(&self) -> bool {
        *self == OA2MSK::Mask4
    }
    ///OA2\[5:1\] are masked and don’t care. Only OA2\[7:6\] are compared
    #[inline(always)]
    pub fn is_mask5(&self) -> bool {
        *self == OA2MSK::Mask5
    }
    ///OA2\[6:1\] are masked and don’t care. Only OA2\[7\] is compared.
    #[inline(always)]
    pub fn is_mask6(&self) -> bool {
        *self == OA2MSK::Mask6
    }
    ///OA2\[7:1\] are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged
    #[inline(always)]
    pub fn is_mask7(&self) -> bool {
        *self == OA2MSK::Mask7
    }
}
///Field `OA2MSK` writer - Own Address 2 masks
pub type OA2MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OA2MSK, crate::Safe>;
impl<'a, REG> OA2MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No mask
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::NoMask)
    }
    ///OA2\[1\] is masked and don’t care. Only OA2\[7:2\] are compared
    #[inline(always)]
    pub fn mask1(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask1)
    }
    ///OA2\[2:1\] are masked and don’t care. Only OA2\[7:3\] are compared
    #[inline(always)]
    pub fn mask2(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask2)
    }
    ///OA2\[3:1\] are masked and don’t care. Only OA2\[7:4\] are compared
    #[inline(always)]
    pub fn mask3(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask3)
    }
    ///OA2\[4:1\] are masked and don’t care. Only OA2\[7:5\] are compared
    #[inline(always)]
    pub fn mask4(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask4)
    }
    ///OA2\[5:1\] are masked and don’t care. Only OA2\[7:6\] are compared
    #[inline(always)]
    pub fn mask5(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask5)
    }
    ///OA2\[6:1\] are masked and don’t care. Only OA2\[7\] is compared.
    #[inline(always)]
    pub fn mask6(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask6)
    }
    ///OA2\[7:1\] are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged
    #[inline(always)]
    pub fn mask7(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask7)
    }
}
/**Own Address 2 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA2EN {
    ///0: Own address 2 disabled. The received slave address OA2 is NACKed
    Disabled = 0,
    ///1: Own address 2 enabled. The received slave address OA2 is ACKed
    Enabled = 1,
}
impl From<OA2EN> for bool {
    #[inline(always)]
    fn from(variant: OA2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OA2EN` reader - Own Address 2 enable
pub type OA2EN_R = crate::BitReader<OA2EN>;
impl OA2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OA2EN {
        match self.bits {
            false => OA2EN::Disabled,
            true => OA2EN::Enabled,
        }
    }
    ///Own address 2 disabled. The received slave address OA2 is NACKed
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA2EN::Disabled
    }
    ///Own address 2 enabled. The received slave address OA2 is ACKed
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA2EN::Enabled
    }
}
///Field `OA2EN` writer - Own Address 2 enable
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG, OA2EN>;
impl<'a, REG> OA2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Own address 2 disabled. The received slave address OA2 is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN::Disabled)
    }
    ///Own address 2 enabled. The received slave address OA2 is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN::Enabled)
    }
}
impl R {
    ///Bits 1:7 - Interface address
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bits 8:10 - Own Address 2 masks
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 15 - Own Address 2 enable
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR2")
            .field("oa2", &self.oa2())
            .field("oa2msk", &self.oa2msk())
            .field("oa2en", &self.oa2en())
            .finish()
    }
}
impl W {
    ///Bits 1:7 - Interface address
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W<OAR2rs> {
        OA2_W::new(self, 1)
    }
    ///Bits 8:10 - Own Address 2 masks
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W<OAR2rs> {
        OA2MSK_W::new(self, 8)
    }
    ///Bit 15 - Own Address 2 enable
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W<OAR2rs> {
        OA2EN_W::new(self, 15)
    }
}
/**Own address register 2

You can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#I2C1:OAR2)*/
pub struct OAR2rs;
impl crate::RegisterSpec for OAR2rs {
    type Ux = u32;
}
///`read()` method returns [`oar2::R`](R) reader structure
impl crate::Readable for OAR2rs {}
///`write(|w| ..)` method takes [`oar2::W`](W) writer structure
impl crate::Writable for OAR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OAR2 to value 0
impl crate::Resettable for OAR2rs {}
