///Register `HSLVR` reader
pub type R = crate::R<HSLVRrs>;
///Register `HSLVR` writer
pub type W = crate::W<HSLVRrs>;
/**Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIGH_SPEED_LOW_VOLTAGE {
    ///0: I/O speed optimization disabled
    Disabled = 0,
    ///1: I/O speed optimization enabled
    Enabled = 1,
}
impl From<HIGH_SPEED_LOW_VOLTAGE> for bool {
    #[inline(always)]
    fn from(variant: HIGH_SPEED_LOW_VOLTAGE) -> Self {
        variant as u8 != 0
    }
}
///Field `HSLV(0-15)` reader - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV_R = crate::BitReader<HIGH_SPEED_LOW_VOLTAGE>;
impl HSLV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HIGH_SPEED_LOW_VOLTAGE {
        match self.bits {
            false => HIGH_SPEED_LOW_VOLTAGE::Disabled,
            true => HIGH_SPEED_LOW_VOLTAGE::Enabled,
        }
    }
    ///I/O speed optimization disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HIGH_SPEED_LOW_VOLTAGE::Disabled
    }
    ///I/O speed optimization enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HIGH_SPEED_LOW_VOLTAGE::Enabled
    }
}
///Field `HSLV(0-15)` writer - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type HSLV_W<'a, REG> = crate::BitWriter<'a, REG, HIGH_SPEED_LOW_VOLTAGE>;
impl<'a, REG> HSLV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I/O speed optimization disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HIGH_SPEED_LOW_VOLTAGE::Disabled)
    }
    ///I/O speed optimization enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HIGH_SPEED_LOW_VOLTAGE::Enabled)
    }
}
impl R {
    ///Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `HSLV0` field.</div>
    #[inline(always)]
    pub fn hslv(&self, n: u8) -> HSLV_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        HSLV_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv_iter(&self) -> impl Iterator<Item = HSLV_R> + '_ {
        (0..16).map(move |n| HSLV_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv0(&self) -> HSLV_R {
        HSLV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv1(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv2(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv3(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv4(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv5(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv6(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv7(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv8(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv9(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv10(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv11(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv12(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv13(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv14(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv15(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSLVR")
            .field("hslv0", &self.hslv0())
            .field("hslv1", &self.hslv1())
            .field("hslv2", &self.hslv2())
            .field("hslv3", &self.hslv3())
            .field("hslv4", &self.hslv4())
            .field("hslv5", &self.hslv5())
            .field("hslv6", &self.hslv6())
            .field("hslv7", &self.hslv7())
            .field("hslv8", &self.hslv8())
            .field("hslv9", &self.hslv9())
            .field("hslv10", &self.hslv10())
            .field("hslv11", &self.hslv11())
            .field("hslv12", &self.hslv12())
            .field("hslv13", &self.hslv13())
            .field("hslv14", &self.hslv14())
            .field("hslv15", &self.hslv15())
            .finish()
    }
}
impl W {
    ///Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `HSLV0` field.</div>
    #[inline(always)]
    pub fn hslv(&mut self, n: u8) -> HSLV_W<HSLVRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        HSLV_W::new(self, n)
    }
    ///Bit 0 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv0(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 0)
    }
    ///Bit 1 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv1(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 1)
    }
    ///Bit 2 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv2(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 2)
    }
    ///Bit 3 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv3(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 3)
    }
    ///Bit 4 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv4(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 4)
    }
    ///Bit 5 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv5(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 5)
    }
    ///Bit 6 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv6(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 6)
    }
    ///Bit 7 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv7(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 7)
    }
    ///Bit 8 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv8(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 8)
    }
    ///Bit 9 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv9(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 9)
    }
    ///Bit 10 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv10(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 10)
    }
    ///Bit 11 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv11(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 11)
    }
    ///Bit 12 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv12(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 12)
    }
    ///Bit 13 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv13(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 13)
    }
    ///Bit 14 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv14(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 14)
    }
    ///Bit 15 - Port x high-speed low-voltage configuration These bits are written by software to optimize the I/O speed when the I/O supply is low. Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit is set. It must be used only if the I/O supply voltage is below 2.7 V. Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be destructive. Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must be kept at reset value. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn hslv15(&mut self) -> HSLV_W<HSLVRrs> {
        HSLV_W::new(self, 15)
    }
}
/**GPIO high-speed low-voltage register

You can [`read`](crate::Reg::read) this register and get [`hslvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hslvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#GPIOA:HSLVR)*/
pub struct HSLVRrs;
impl crate::RegisterSpec for HSLVRrs {
    type Ux = u32;
}
///`read()` method returns [`hslvr::R`](R) reader structure
impl crate::Readable for HSLVRrs {}
///`write(|w| ..)` method takes [`hslvr::W`](W) writer structure
impl crate::Writable for HSLVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HSLVR to value 0
impl crate::Resettable for HSLVRrs {}
