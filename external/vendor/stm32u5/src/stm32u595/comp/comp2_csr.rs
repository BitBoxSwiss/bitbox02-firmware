///Register `COMP2_CSR` reader
pub type R = crate::R<COMP2_CSRrs>;
///Register `COMP2_CSR` writer
pub type W = crate::W<COMP2_CSRrs>;
///Field `COM2_EN` reader - Comparator 2 enable bit
pub type COM2_EN_R = crate::BitReader;
///Field `COM2_EN` writer - Comparator 2 enable bit
pub type COM2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COM2_INMSEL` reader - Comparator 2 Input Minus connection configuration bit
pub type COM2_INMSEL_R = crate::FieldReader;
///Field `COM2_INMSEL` writer - Comparator 2 Input Minus connection configuration bit
pub type COM2_INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COM2_INPSEL` reader - Comparator 2 input plus selection bit
pub type COM2_INPSEL_R = crate::FieldReader;
///Field `COM2_INPSEL` writer - Comparator 2 input plus selection bit
pub type COM2_INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COM2_WINMODE` reader - COM2_WINMODE
pub type COM2_WINMODE_R = crate::BitReader;
///Field `COM2_WINMODE` writer - COM2_WINMODE
pub type COM2_WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COM2_WINOUT` reader - COM2_WINOUT
pub type COM2_WINOUT_R = crate::BitReader;
///Field `COM2_WINOUT` writer - COM2_WINOUT
pub type COM2_WINOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COM2_POLARITY` reader - Comparator 2 polarity selection bit
pub type COM2_POLARITY_R = crate::BitReader;
///Field `COM2_POLARITY` writer - Comparator 2 polarity selection bit
pub type COM2_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COM2_HYST` reader - Comparator 2 hysteresis selection bits
pub type COM2_HYST_R = crate::FieldReader;
///Field `COM2_HYST` writer - Comparator 2 hysteresis selection bits
pub type COM2_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COM2_PWRMODE` reader - COM2_PWRMODE
pub type COM2_PWRMODE_R = crate::FieldReader;
///Field `COM2_PWRMODE` writer - COM2_PWRMODE
pub type COM2_PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COM2_BLANKSEL` reader - COM2_BLANKSEL
pub type COM2_BLANKSEL_R = crate::FieldReader;
///Field `COM2_BLANKSEL` writer - COM2_BLANKSEL
pub type COM2_BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `COM2_VALUE` reader - Comparator 2 output status bit
pub type COM2_VALUE_R = crate::BitReader;
///Field `COM2_LOCK` reader - COMP2_CSR register lock bit
pub type COM2_LOCK_R = crate::BitReader;
///Field `COM2_LOCK` writer - COMP2_CSR register lock bit
pub type COM2_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn com2_en(&self) -> COM2_EN_R {
        COM2_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Comparator 2 Input Minus connection configuration bit
    #[inline(always)]
    pub fn com2_inmsel(&self) -> COM2_INMSEL_R {
        COM2_INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator 2 input plus selection bit
    #[inline(always)]
    pub fn com2_inpsel(&self) -> COM2_INPSEL_R {
        COM2_INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - COM2_WINMODE
    #[inline(always)]
    pub fn com2_winmode(&self) -> COM2_WINMODE_R {
        COM2_WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - COM2_WINOUT
    #[inline(always)]
    pub fn com2_winout(&self) -> COM2_WINOUT_R {
        COM2_WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn com2_polarity(&self) -> COM2_POLARITY_R {
        COM2_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn com2_hyst(&self) -> COM2_HYST_R {
        COM2_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - COM2_PWRMODE
    #[inline(always)]
    pub fn com2_pwrmode(&self) -> COM2_PWRMODE_R {
        COM2_PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - COM2_BLANKSEL
    #[inline(always)]
    pub fn com2_blanksel(&self) -> COM2_BLANKSEL_R {
        COM2_BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 2 output status bit
    #[inline(always)]
    pub fn com2_value(&self) -> COM2_VALUE_R {
        COM2_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP2_CSR register lock bit
    #[inline(always)]
    pub fn com2_lock(&self) -> COM2_LOCK_R {
        COM2_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2_CSR")
            .field("com2_en", &self.com2_en())
            .field("com2_inmsel", &self.com2_inmsel())
            .field("com2_inpsel", &self.com2_inpsel())
            .field("com2_winmode", &self.com2_winmode())
            .field("com2_winout", &self.com2_winout())
            .field("com2_polarity", &self.com2_polarity())
            .field("com2_hyst", &self.com2_hyst())
            .field("com2_pwrmode", &self.com2_pwrmode())
            .field("com2_blanksel", &self.com2_blanksel())
            .field("com2_value", &self.com2_value())
            .field("com2_lock", &self.com2_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn com2_en(&mut self) -> COM2_EN_W<COMP2_CSRrs> {
        COM2_EN_W::new(self, 0)
    }
    ///Bits 4:7 - Comparator 2 Input Minus connection configuration bit
    #[inline(always)]
    pub fn com2_inmsel(&mut self) -> COM2_INMSEL_W<COMP2_CSRrs> {
        COM2_INMSEL_W::new(self, 4)
    }
    ///Bits 8:9 - Comparator 2 input plus selection bit
    #[inline(always)]
    pub fn com2_inpsel(&mut self) -> COM2_INPSEL_W<COMP2_CSRrs> {
        COM2_INPSEL_W::new(self, 8)
    }
    ///Bit 11 - COM2_WINMODE
    #[inline(always)]
    pub fn com2_winmode(&mut self) -> COM2_WINMODE_W<COMP2_CSRrs> {
        COM2_WINMODE_W::new(self, 11)
    }
    ///Bit 14 - COM2_WINOUT
    #[inline(always)]
    pub fn com2_winout(&mut self) -> COM2_WINOUT_W<COMP2_CSRrs> {
        COM2_WINOUT_W::new(self, 14)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn com2_polarity(&mut self) -> COM2_POLARITY_W<COMP2_CSRrs> {
        COM2_POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn com2_hyst(&mut self) -> COM2_HYST_W<COMP2_CSRrs> {
        COM2_HYST_W::new(self, 16)
    }
    ///Bits 18:19 - COM2_PWRMODE
    #[inline(always)]
    pub fn com2_pwrmode(&mut self) -> COM2_PWRMODE_W<COMP2_CSRrs> {
        COM2_PWRMODE_W::new(self, 18)
    }
    ///Bits 20:24 - COM2_BLANKSEL
    #[inline(always)]
    pub fn com2_blanksel(&mut self) -> COM2_BLANKSEL_W<COMP2_CSRrs> {
        COM2_BLANKSEL_W::new(self, 20)
    }
    ///Bit 31 - COMP2_CSR register lock bit
    #[inline(always)]
    pub fn com2_lock(&mut self) -> COM2_LOCK_W<COMP2_CSRrs> {
        COM2_LOCK_W::new(self, 31)
    }
}
/**Comparator 2 control and status register

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#COMP:COMP2_CSR)*/
pub struct COMP2_CSRrs;
impl crate::RegisterSpec for COMP2_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp2_csr::R`](R) reader structure
impl crate::Readable for COMP2_CSRrs {}
///`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure
impl crate::Writable for COMP2_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP2_CSR to value 0
impl crate::Resettable for COMP2_CSRrs {}
