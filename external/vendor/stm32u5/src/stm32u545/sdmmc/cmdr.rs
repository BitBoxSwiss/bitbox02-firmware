///Register `CMDR` reader
pub type R = crate::R<CMDRrs>;
///Register `CMDR` writer
pub type W = crate::W<CMDRrs>;
///Field `CMDINDEX` reader - Command index
pub type CMDINDEX_R = crate::FieldReader;
///Field `CMDINDEX` writer - Command index
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CMDTRANS` reader - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM
pub type CMDTRANS_R = crate::BitReader;
///Field `CMDTRANS` writer - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM
pub type CMDTRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTOP` reader - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM
pub type CMDSTOP_R = crate::BitReader;
///Field `CMDSTOP` writer - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM
pub type CMDSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITRESP` reader - Wait for response bits
pub type WAITRESP_R = crate::FieldReader;
///Field `WAITRESP` writer - Wait for response bits
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WAITINT` reader - CPSM waits for interrupt request
pub type WAITINT_R = crate::BitReader;
///Field `WAITINT` writer - CPSM waits for interrupt request
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal) from DPSM
pub type WAITPEND_R = crate::BitReader;
///Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal) from DPSM
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit
pub type CPSMEN_R = crate::BitReader;
///Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTHOLD` reader - Hold new data block transmission and reception in the DPSM
pub type DTHOLD_R = crate::BitReader;
///Field `DTHOLD` writer - Hold new data block transmission and reception in the DPSM
pub type DTHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTMODE` reader - Select the boot mode procedure to be used
pub type BOOTMODE_R = crate::BitReader;
///Field `BOOTMODE` writer - Select the boot mode procedure to be used
pub type BOOTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTEN` reader - Enable boot mode procedure
pub type BOOTEN_R = crate::BitReader;
///Field `BOOTEN` writer - Enable boot mode procedure
pub type BOOTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSUSPEND` reader - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end
pub type CMDSUSPEND_R = crate::BitReader;
///Field `CMDSUSPEND` writer - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end
pub type CMDSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Command index
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Wait for response bits
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - CPSM waits for interrupt request
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPSM Waits for ends of data transfer (CmdPend internal signal) from DPSM
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Command path state machine (CPSM) Enable bit
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hold new data block transmission and reception in the DPSM
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Select the boot mode procedure to be used
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable boot mode procedure
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDR")
            .field("cmdsuspend", &self.cmdsuspend())
            .field("booten", &self.booten())
            .field("bootmode", &self.bootmode())
            .field("dthold", &self.dthold())
            .field("cpsmen", &self.cpsmen())
            .field("waitpend", &self.waitpend())
            .field("waitint", &self.waitint())
            .field("waitresp", &self.waitresp())
            .field("cmdstop", &self.cmdstop())
            .field("cmdtrans", &self.cmdtrans())
            .field("cmdindex", &self.cmdindex())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Command index
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDRrs> {
        CMDINDEX_W::new(self, 0)
    }
    ///Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM
    #[inline(always)]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<CMDRrs> {
        CMDTRANS_W::new(self, 6)
    }
    ///Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM
    #[inline(always)]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<CMDRrs> {
        CMDSTOP_W::new(self, 7)
    }
    ///Bits 8:9 - Wait for response bits
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDRrs> {
        WAITRESP_W::new(self, 8)
    }
    ///Bit 10 - CPSM waits for interrupt request
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W<CMDRrs> {
        WAITINT_W::new(self, 10)
    }
    ///Bit 11 - CPSM Waits for ends of data transfer (CmdPend internal signal) from DPSM
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDRrs> {
        WAITPEND_W::new(self, 11)
    }
    ///Bit 12 - Command path state machine (CPSM) Enable bit
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDRrs> {
        CPSMEN_W::new(self, 12)
    }
    ///Bit 13 - Hold new data block transmission and reception in the DPSM
    #[inline(always)]
    pub fn dthold(&mut self) -> DTHOLD_W<CMDRrs> {
        DTHOLD_W::new(self, 13)
    }
    ///Bit 14 - Select the boot mode procedure to be used
    #[inline(always)]
    pub fn bootmode(&mut self) -> BOOTMODE_W<CMDRrs> {
        BOOTMODE_W::new(self, 14)
    }
    ///Bit 15 - Enable boot mode procedure
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W<CMDRrs> {
        BOOTEN_W::new(self, 15)
    }
    ///Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end
    #[inline(always)]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W<CMDRrs> {
        CMDSUSPEND_W::new(self, 16)
    }
}
/**command register

You can [`read`](crate::Reg::read) this register and get [`cmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:CMDR)*/
pub struct CMDRrs;
impl crate::RegisterSpec for CMDRrs {
    type Ux = u32;
}
///`read()` method returns [`cmdr::R`](R) reader structure
impl crate::Readable for CMDRrs {}
///`write(|w| ..)` method takes [`cmdr::W`](W) writer structure
impl crate::Writable for CMDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMDR to value 0
impl crate::Resettable for CMDRrs {}
