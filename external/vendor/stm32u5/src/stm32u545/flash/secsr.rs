///Register `SECSR` reader
pub type R = crate::R<SECSRrs>;
///Register `SECSR` writer
pub type W = crate::W<SECSRrs>;
///Field `EOP` reader - Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1.
pub type EOP_R = crate::BitReader;
///Field `EOP` writer - Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1.
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERR` reader - Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1.
pub type OPERR_R = crate::BitReader;
///Field `OPERR` writer - Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1.
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROGERR` reader - Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
pub type PROGERR_R = crate::BitReader;
///Field `PROGERR` writer - Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERR` reader - Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type WRPERR_R = crate::BitReader;
///Field `WRPERR` writer - Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGAERR` reader - Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1.
pub type PGAERR_R = crate::BitReader;
///Field `PGAERR` writer - Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1.
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIZERR` reader - Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1.
pub type SIZERR_R = crate::BitReader;
///Field `SIZERR` writer - Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1.
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERR` reader - Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type PGSERR_R = crate::BitReader;
///Field `PGSERR` writer - Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDERR` reader - Secure readout protection error This bit is set by hardware when a read access is performed to a secure PCROP area and when a cacheable fetch access is performed to a secure PCROP area. An interrupt is generated if RDERRIE is set in FLASH_SECCR register. This bit is cleared by writing 1.
pub type RDERR_R = crate::BitReader;
///Field `RDERR` writer - Secure readout protection error This bit is set by hardware when a read access is performed to a secure PCROP area and when a cacheable fetch access is performed to a secure PCROP area. An interrupt is generated if RDERRIE is set in FLASH_SECCR register. This bit is cleared by writing 1.
pub type RDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSY` reader - Secure busy This bit indicates that a Flash memory secure or non-secure operation is in progress. This is set on the beginning of a Flash operation and reset when the operation finishes or when an error occurs.
pub type BSY_R = crate::BitReader;
///Field `WDW` reader - Secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory.
pub type WDW_R = crate::BitReader;
impl R {
    ///Bit 0 - Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1.
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1.
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 14 - Secure readout protection error This bit is set by hardware when a read access is performed to a secure PCROP area and when a cacheable fetch access is performed to a secure PCROP area. An interrupt is generated if RDERRIE is set in FLASH_SECCR register. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Secure busy This bit indicates that a Flash memory secure or non-secure operation is in progress. This is set on the beginning of a Flash operation and reset when the operation finishes or when an error occurs.
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory.
    #[inline(always)]
    pub fn wdw(&self) -> WDW_R {
        WDW_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECSR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("progerr", &self.progerr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("pgserr", &self.pgserr())
            .field("rderr", &self.rderr())
            .field("bsy", &self.bsy())
            .field("wdw", &self.wdw())
            .finish()
    }
}
impl W {
    ///Bit 0 - Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<SECSRrs> {
        EOP_W::new(self, 0)
    }
    ///Bit 1 - Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<SECSRrs> {
        OPERR_W::new(self, 1)
    }
    ///Bit 3 - Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W<SECSRrs> {
        PROGERR_W::new(self, 3)
    }
    ///Bit 4 - Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<SECSRrs> {
        WRPERR_W::new(self, 4)
    }
    ///Bit 5 - Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1.
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<SECSRrs> {
        PGAERR_W::new(self, 5)
    }
    ///Bit 6 - Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1.
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<SECSRrs> {
        SIZERR_W::new(self, 6)
    }
    ///Bit 7 - Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<SECSRrs> {
        PGSERR_W::new(self, 7)
    }
    ///Bit 14 - Secure readout protection error This bit is set by hardware when a read access is performed to a secure PCROP area and when a cacheable fetch access is performed to a secure PCROP area. An interrupt is generated if RDERRIE is set in FLASH_SECCR register. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W<SECSRrs> {
        RDERR_W::new(self, 14)
    }
}
/**FLASH secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FLASH:SECSR)*/
pub struct SECSRrs;
impl crate::RegisterSpec for SECSRrs {
    type Ux = u32;
}
///`read()` method returns [`secsr::R`](R) reader structure
impl crate::Readable for SECSRrs {}
///`write(|w| ..)` method takes [`secsr::W`](W) writer structure
impl crate::Writable for SECSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECSR to value 0
impl crate::Resettable for SECSRrs {}
