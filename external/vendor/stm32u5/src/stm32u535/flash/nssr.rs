///Register `NSSR` reader
pub type R = crate::R<NSSRrs>;
///Register `NSSR` writer
pub type W = crate::W<NSSRrs>;
///Field `EOP` reader - Non-secure end of operation This bit is set by hardware when one or more Flash memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_NSCR). This bit is cleared by writing 1.
pub type EOP_R = crate::BitReader;
///Field `EOP` writer - Non-secure end of operation This bit is set by hardware when one or more Flash memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_NSCR). This bit is cleared by writing 1.
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERR` reader - Non-secure operation error This bit is set by hardware when a Flash memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1.
pub type OPERR_R = crate::BitReader;
///Field `OPERR` writer - Non-secure operation error This bit is set by hardware when a Flash memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1.
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROGERR` reader - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
pub type PROGERR_R = crate::BitReader;
///Field `PROGERR` writer - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRPERR` reader - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type WRPERR_R = crate::BitReader;
///Field `WRPERR` writer - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGAERR` reader - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
pub type PGAERR_R = crate::BitReader;
///Field `PGAERR` writer - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIZERR` reader - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
pub type SIZERR_R = crate::BitReader;
///Field `SIZERR` writer - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGSERR` reader - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type PGSERR_R = crate::BitReader;
///Field `PGSERR` writer - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTWERR` reader - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type OPTWERR_R = crate::BitReader;
///Field `OPTWERR` writer - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type OPTWERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSY` reader - Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs.
pub type BSY_R = crate::BitReader;
///Field `WDW` reader - Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory.
pub type WDW_R = crate::BitReader;
///Field `OEM1LOCK` reader - OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active.
pub type OEM1LOCK_R = crate::BitReader;
///Field `OEM2LOCK` reader - OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active.
pub type OEM2LOCK_R = crate::BitReader;
///Field `PD1` reader - Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken.
pub type PD1_R = crate::BitReader;
///Field `PD2` reader - Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken.
pub type PD2_R = crate::BitReader;
impl R {
    ///Bit 0 - Non-secure end of operation This bit is set by hardware when one or more Flash memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_NSCR). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Non-secure operation error This bit is set by hardware when a Flash memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs.
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory.
    #[inline(always)]
    pub fn wdw(&self) -> WDW_R {
        WDW_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active.
    #[inline(always)]
    pub fn oem1lock(&self) -> OEM1LOCK_R {
        OEM1LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active.
    #[inline(always)]
    pub fn oem2lock(&self) -> OEM2LOCK_R {
        OEM2LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken.
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken.
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSSR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("progerr", &self.progerr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("pgserr", &self.pgserr())
            .field("optwerr", &self.optwerr())
            .field("bsy", &self.bsy())
            .field("wdw", &self.wdw())
            .field("oem1lock", &self.oem1lock())
            .field("oem2lock", &self.oem2lock())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non-secure end of operation This bit is set by hardware when one or more Flash memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_NSCR). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<NSSRrs> {
        EOP_W::new(self, 0)
    }
    ///Bit 1 - Non-secure operation error This bit is set by hardware when a Flash memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1.
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<NSSRrs> {
        OPERR_W::new(self, 1)
    }
    ///Bit 3 - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W<NSSRrs> {
        PROGERR_W::new(self, 3)
    }
    ///Bit 4 - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<NSSRrs> {
        WRPERR_W::new(self, 4)
    }
    ///Bit 5 - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<NSSRrs> {
        PGAERR_W::new(self, 5)
    }
    ///Bit 6 - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<NSSRrs> {
        SIZERR_W::new(self, 6)
    }
    ///Bit 7 - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<NSSRrs> {
        PGSERR_W::new(self, 7)
    }
    ///Bit 13 - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn optwerr(&mut self) -> OPTWERR_W<NSSRrs> {
        OPTWERR_W::new(self, 13)
    }
}
/**FLASH non-secure status register

You can [`read`](crate::Reg::read) this register and get [`nssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FLASH:NSSR)*/
pub struct NSSRrs;
impl crate::RegisterSpec for NSSRrs {
    type Ux = u32;
}
///`read()` method returns [`nssr::R`](R) reader structure
impl crate::Readable for NSSRrs {}
///`write(|w| ..)` method takes [`nssr::W`](W) writer structure
impl crate::Writable for NSSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSSR to value 0
impl crate::Resettable for NSSRrs {}
