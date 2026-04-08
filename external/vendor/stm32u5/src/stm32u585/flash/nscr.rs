///Register `NSCR` reader
pub type R = crate::R<NSCRrs>;
///Register `NSCR` writer
pub type W = crate::W<NSCRrs>;
///Field `PG` reader - Non-secure programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Non-secure programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Non-secure page erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Non-secure page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER1` reader - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
pub type MER1_R = crate::BitReader;
///Field `MER1` writer - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Non-secure page number selection These bits select the page to erase. ...
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Non-secure page number selection These bits select the page to erase. ...
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `BKER` reader - Non-secure bank selection for page erase
pub type BKER_R = crate::BitReader;
///Field `BKER` writer - Non-secure bank selection for page erase
pub type BKER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BWR` reader - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_R = crate::BitReader;
///Field `BWR` writer - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER2` reader - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
pub type MER2_R = crate::BitReader;
///Field `MER2` writer - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBL_LAUNCH` reader - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
pub type OBL_LAUNCH_R = crate::BitReader;
///Field `OBL_LAUNCH` writer - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTLOCK` reader - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Non-secure programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Non-secure page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Non-secure page number selection These bits select the page to erase. ...
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - Non-secure bank selection for page erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSCR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer1", &self.mer1())
            .field("pnb", &self.pnb())
            .field("bker", &self.bker())
            .field("bwr", &self.bwr())
            .field("mer2", &self.mer2())
            .field("strt", &self.strt())
            .field("optstrt", &self.optstrt())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("obl_launch", &self.obl_launch())
            .field("optlock", &self.optlock())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non-secure programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<NSCRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Non-secure page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<NSCRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W<NSCRrs> {
        MER1_W::new(self, 2)
    }
    ///Bits 3:9 - Non-secure page number selection These bits select the page to erase. ...
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<NSCRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 11 - Non-secure bank selection for page erase
    #[inline(always)]
    pub fn bker(&mut self) -> BKER_W<NSCRrs> {
        BKER_W::new(self, 11)
    }
    ///Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&mut self) -> BWR_W<NSCRrs> {
        BWR_W::new(self, 14)
    }
    ///Bit 15 - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W<NSCRrs> {
        MER2_W::new(self, 15)
    }
    ///Bit 16 - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<NSCRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 17 - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<NSCRrs> {
        OPTSTRT_W::new(self, 17)
    }
    ///Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<NSCRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<NSCRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set.
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<NSCRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    ///Bit 30 - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<NSCRrs> {
        OPTLOCK_W::new(self, 30)
    }
    ///Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<NSCRrs> {
        LOCK_W::new(self, 31)
    }
}
/**FLASH non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FLASH:NSCR)*/
pub struct NSCRrs;
impl crate::RegisterSpec for NSCRrs {
    type Ux = u32;
}
///`read()` method returns [`nscr::R`](R) reader structure
impl crate::Readable for NSCRrs {}
///`write(|w| ..)` method takes [`nscr::W`](W) writer structure
impl crate::Writable for NSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSCR to value 0xc000_0000
impl crate::Resettable for NSCRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
