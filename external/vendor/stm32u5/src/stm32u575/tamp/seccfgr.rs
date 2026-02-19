///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `BKPRWSEC` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
pub type BKPRWSEC_R = crate::FieldReader;
///Field `BKPRWSEC` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
pub type BKPRWSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CNT1SEC` reader - Monotonic counter 1 secure protection
pub type CNT1SEC_R = crate::BitReader;
///Field `CNT1SEC` writer - Monotonic counter 1 secure protection
pub type CNT1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPWSEC` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ¥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ¤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
pub type BKPWSEC_R = crate::FieldReader;
///Field `BKPWSEC` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ¥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ¤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
pub type BKPWSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BHKLOCK` reader - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
pub type BHKLOCK_R = crate::BitReader;
///Field `BHKLOCK` writer - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
pub type BHKLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPSEC` reader - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection.
pub type TAMPSEC_R = crate::BitReader;
///Field `TAMPSEC` writer - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection.
pub type TAMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprwsec(&self) -> BKPRWSEC_R {
        BKPRWSEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 15 - Monotonic counter 1 secure protection
    #[inline(always)]
    pub fn cnt1sec(&self) -> CNT1SEC_R {
        CNT1SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ¥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ¤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkpwsec(&self) -> BKPWSEC_R {
        BKPWSEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bhklock(&self) -> BHKLOCK_R {
        BHKLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection.
    #[inline(always)]
    pub fn tampsec(&self) -> TAMPSEC_R {
        TAMPSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("bkprwsec", &self.bkprwsec())
            .field("cnt1sec", &self.cnt1sec())
            .field("bkpwsec", &self.bkpwsec())
            .field("bhklock", &self.bhklock())
            .field("tampsec", &self.tampsec())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprwsec(&mut self) -> BKPRWSEC_W<SECCFGRrs> {
        BKPRWSEC_W::new(self, 0)
    }
    ///Bit 15 - Monotonic counter 1 secure protection
    #[inline(always)]
    pub fn cnt1sec(&mut self) -> CNT1SEC_W<SECCFGRrs> {
        CNT1SEC_W::new(self, 15)
    }
    ///Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ¥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ¤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\[7:0\] can be written only in privileged mode.
    #[inline(always)]
    pub fn bkpwsec(&mut self) -> BKPWSEC_W<SECCFGRrs> {
        BKPWSEC_W::new(self, 16)
    }
    ///Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bhklock(&mut self) -> BHKLOCK_W<SECCFGRrs> {
        BHKLOCK_W::new(self, 30)
    }
    ///Bit 31 - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection.
    #[inline(always)]
    pub fn tampsec(&mut self) -> TAMPSEC_W<SECCFGRrs> {
        TAMPSEC_W::new(self, 31)
    }
}
/**TAMP secure mode register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
