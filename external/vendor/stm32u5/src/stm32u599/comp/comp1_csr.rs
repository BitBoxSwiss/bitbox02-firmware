///Register `COMP1_CSR` reader
pub type R = crate::R<COMP1_CSRrs>;
///Register `COMP1_CSR` writer
pub type W = crate::W<COMP1_CSRrs>;
///Field `COMP1_EN` reader - Comparator 1 enable bit
pub type COMP1_EN_R = crate::BitReader;
///Field `COMP1_EN` writer - Comparator 1 enable bit
pub type COMP1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_INMSEL` reader - Comparator 1 Input Minus connection configuration bit
pub type COMP1_INMSEL_R = crate::FieldReader;
///Field `COMP1_INMSEL` writer - Comparator 1 Input Minus connection configuration bit
pub type COMP1_INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COMP1_INPSEL` reader - Comparator1 input plus selection bit
pub type COMP1_INPSEL_R = crate::FieldReader;
///Field `COMP1_INPSEL` writer - Comparator1 input plus selection bit
pub type COMP1_INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_WINMODE` reader - COMP1_WINMODE
pub type COMP1_WINMODE_R = crate::BitReader;
///Field `COMP1_WINMODE` writer - COMP1_WINMODE
pub type COMP1_WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_WINOUT` reader - COMP1_WINOUT
pub type COMP1_WINOUT_R = crate::BitReader;
///Field `COMP1_WINOUT` writer - COMP1_WINOUT
pub type COMP1_WINOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_POLARITY` reader - Comparator 1 polarity selection bit
pub type COMP1_POLARITY_R = crate::BitReader;
///Field `COMP1_POLARITY` writer - Comparator 1 polarity selection bit
pub type COMP1_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_HYST` reader - Comparator 1 hysteresis selection bits
pub type COMP1_HYST_R = crate::FieldReader;
///Field `COMP1_HYST` writer - Comparator 1 hysteresis selection bits
pub type COMP1_HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_PWRMODE` reader - COMP1_PWRMODE
pub type COMP1_PWRMODE_R = crate::FieldReader;
///Field `COMP1_PWRMODE` writer - COMP1_PWRMODE
pub type COMP1_PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMP1_BLANKSEL` reader - COMP1_BLANKSEL
pub type COMP1_BLANKSEL_R = crate::FieldReader;
///Field `COMP1_BLANKSEL` writer - COMP1_BLANKSEL
pub type COMP1_BLANKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `COMP1_VALUE` reader - Comparator 1 output status bit
pub type COMP1_VALUE_R = crate::BitReader;
///Field `COMP1_LOCK` reader - COMP1_CSR register lock bit
pub type COMP1_LOCK_R = crate::BitReader;
///Field `COMP1_LOCK` writer - COMP1_CSR register lock bit
pub type COMP1_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Comparator 1 enable bit
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Comparator 1 Input Minus connection configuration bit
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> COMP1_INMSEL_R {
        COMP1_INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator1 input plus selection bit
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> COMP1_INPSEL_R {
        COMP1_INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - COMP1_WINMODE
    #[inline(always)]
    pub fn comp1_winmode(&self) -> COMP1_WINMODE_R {
        COMP1_WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - COMP1_WINOUT
    #[inline(always)]
    pub fn comp1_winout(&self) -> COMP1_WINOUT_R {
        COMP1_WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 1 polarity selection bit
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selection bits
    #[inline(always)]
    pub fn comp1_hyst(&self) -> COMP1_HYST_R {
        COMP1_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - COMP1_PWRMODE
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> COMP1_PWRMODE_R {
        COMP1_PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - COMP1_BLANKSEL
    #[inline(always)]
    pub fn comp1_blanksel(&self) -> COMP1_BLANKSEL_R {
        COMP1_BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 1 output status bit
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP1_CSR register lock bit
    #[inline(always)]
    pub fn comp1_lock(&self) -> COMP1_LOCK_R {
        COMP1_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1_CSR")
            .field("comp1_en", &self.comp1_en())
            .field("comp1_inmsel", &self.comp1_inmsel())
            .field("comp1_inpsel", &self.comp1_inpsel())
            .field("comp1_winmode", &self.comp1_winmode())
            .field("comp1_winout", &self.comp1_winout())
            .field("comp1_polarity", &self.comp1_polarity())
            .field("comp1_hyst", &self.comp1_hyst())
            .field("comp1_pwrmode", &self.comp1_pwrmode())
            .field("comp1_blanksel", &self.comp1_blanksel())
            .field("comp1_value", &self.comp1_value())
            .field("comp1_lock", &self.comp1_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable bit
    #[inline(always)]
    pub fn comp1_en(&mut self) -> COMP1_EN_W<COMP1_CSRrs> {
        COMP1_EN_W::new(self, 0)
    }
    ///Bits 4:7 - Comparator 1 Input Minus connection configuration bit
    #[inline(always)]
    pub fn comp1_inmsel(&mut self) -> COMP1_INMSEL_W<COMP1_CSRrs> {
        COMP1_INMSEL_W::new(self, 4)
    }
    ///Bits 8:9 - Comparator1 input plus selection bit
    #[inline(always)]
    pub fn comp1_inpsel(&mut self) -> COMP1_INPSEL_W<COMP1_CSRrs> {
        COMP1_INPSEL_W::new(self, 8)
    }
    ///Bit 11 - COMP1_WINMODE
    #[inline(always)]
    pub fn comp1_winmode(&mut self) -> COMP1_WINMODE_W<COMP1_CSRrs> {
        COMP1_WINMODE_W::new(self, 11)
    }
    ///Bit 14 - COMP1_WINOUT
    #[inline(always)]
    pub fn comp1_winout(&mut self) -> COMP1_WINOUT_W<COMP1_CSRrs> {
        COMP1_WINOUT_W::new(self, 14)
    }
    ///Bit 15 - Comparator 1 polarity selection bit
    #[inline(always)]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W<COMP1_CSRrs> {
        COMP1_POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selection bits
    #[inline(always)]
    pub fn comp1_hyst(&mut self) -> COMP1_HYST_W<COMP1_CSRrs> {
        COMP1_HYST_W::new(self, 16)
    }
    ///Bits 18:19 - COMP1_PWRMODE
    #[inline(always)]
    pub fn comp1_pwrmode(&mut self) -> COMP1_PWRMODE_W<COMP1_CSRrs> {
        COMP1_PWRMODE_W::new(self, 18)
    }
    ///Bits 20:24 - COMP1_BLANKSEL
    #[inline(always)]
    pub fn comp1_blanksel(&mut self) -> COMP1_BLANKSEL_W<COMP1_CSRrs> {
        COMP1_BLANKSEL_W::new(self, 20)
    }
    ///Bit 31 - COMP1_CSR register lock bit
    #[inline(always)]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W<COMP1_CSRrs> {
        COMP1_LOCK_W::new(self, 31)
    }
}
/**Comparator 1 control and status register

You can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#COMP:COMP1_CSR)*/
pub struct COMP1_CSRrs;
impl crate::RegisterSpec for COMP1_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp1_csr::R`](R) reader structure
impl crate::Readable for COMP1_CSRrs {}
///`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure
impl crate::Writable for COMP1_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP1_CSR to value 0
impl crate::Resettable for COMP1_CSRrs {}
