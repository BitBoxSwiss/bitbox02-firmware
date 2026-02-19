///Register `SECCR` reader
pub type R = crate::R<SECCRrs>;
///Register `SECCR` writer
pub type W = crate::W<SECCRrs>;
///Field `PG` reader - Secure programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Secure programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Secure page erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Secure page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER1` reader - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
pub type MER1_R = crate::BitReader;
///Field `MER1` writer - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Secure page number selection These bits select the page to erase: ...
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Secure page number selection These bits select the page to erase: ...
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `BKER` reader - Secure bank selection for page erase
pub type BKER_R = crate::BitReader;
///Field `BKER` writer - Secure bank selection for page erase
pub type BKER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BWR` reader - Secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_R = crate::BitReader;
///Field `BWR` writer - Secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER2` reader - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
pub type MER2_R = crate::BitReader;
///Field `MER2` writer - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Secure error interrupt enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Secure error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDERRIE` reader - Secure PCROP read error interrupt enable
pub type RDERRIE_R = crate::BitReader;
///Field `RDERRIE` writer - Secure PCROP read error interrupt enable
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INV` reader - Flash memory security state invert This bit inverts the Flash memory security state.
pub type INV_R = crate::BitReader;
///Field `INV` writer - Flash memory security state invert This bit inverts the Flash memory security state.
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Secure programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Secure page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Secure page number selection These bits select the page to erase: ...
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - Secure bank selection for page erase
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Secure error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Secure PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - Flash memory security state invert This bit inverts the Flash memory security state.
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer1", &self.mer1())
            .field("pnb", &self.pnb())
            .field("bker", &self.bker())
            .field("bwr", &self.bwr())
            .field("mer2", &self.mer2())
            .field("strt", &self.strt())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("rderrie", &self.rderrie())
            .field("inv", &self.inv())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Secure programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<SECCRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Secure page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<SECCRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W<SECCRrs> {
        MER1_W::new(self, 2)
    }
    ///Bits 3:9 - Secure page number selection These bits select the page to erase: ...
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<SECCRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 11 - Secure bank selection for page erase
    #[inline(always)]
    pub fn bker(&mut self) -> BKER_W<SECCRrs> {
        BKER_W::new(self, 11)
    }
    ///Bit 14 - Secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&mut self) -> BWR_W<SECCRrs> {
        BWR_W::new(self, 14)
    }
    ///Bit 15 - Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W<SECCRrs> {
        MER2_W::new(self, 15)
    }
    ///Bit 16 - Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<SECCRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 24 - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<SECCRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Secure error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<SECCRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 26 - Secure PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<SECCRrs> {
        RDERRIE_W::new(self, 26)
    }
    ///Bit 29 - Flash memory security state invert This bit inverts the Flash memory security state.
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<SECCRrs> {
        INV_W::new(self, 29)
    }
    ///Bit 31 - Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<SECCRrs> {
        LOCK_W::new(self, 31)
    }
}
/**FLASH secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FLASH:SECCR)*/
pub struct SECCRrs;
impl crate::RegisterSpec for SECCRrs {
    type Ux = u32;
}
///`read()` method returns [`seccr::R`](R) reader structure
impl crate::Readable for SECCRrs {}
///`write(|w| ..)` method takes [`seccr::W`](W) writer structure
impl crate::Writable for SECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCR to value 0x8000_0000
impl crate::Resettable for SECCRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
