///Register `CCCR` reader
pub type R = crate::R<CCCRrs>;
///Register `CCCR` writer
pub type W = crate::W<CCCRrs>;
///Field `INIT` reader - Initialization
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialization
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCE` reader - Configuration Change Enable
pub type CCE_R = crate::BitReader;
///Field `CCE` writer - Configuration Change Enable
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASM` reader - ASM Restricted Operation Mode
pub type ASM_R = crate::BitReader;
///Field `ASM` writer - ASM Restricted Operation Mode
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSA` reader - Clock Stop Acknowledge
pub type CSA_R = crate::BitReader;
///Field `CSA` writer - Clock Stop Acknowledge
pub type CSA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSR` reader - Clock Stop Request
pub type CSR_R = crate::BitReader;
///Field `CSR` writer - Clock Stop Request
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MON` reader - Bus Monitoring Mode
pub type MON_R = crate::BitReader;
///Field `MON` writer - Bus Monitoring Mode
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAR` reader - Disable Automatic Retransmission
pub type DAR_R = crate::BitReader;
///Field `DAR` writer - Disable Automatic Retransmission
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEST` reader - Test Mode Enable
pub type TEST_R = crate::BitReader;
///Field `TEST` writer - Test Mode Enable
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDOE` reader - FD Operation Enable
pub type FDOE_R = crate::BitReader;
///Field `FDOE` writer - FD Operation Enable
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRSE` reader - FDCAN Bit Rate Switching
pub type BRSE_R = crate::BitReader;
///Field `BRSE` writer - FDCAN Bit Rate Switching
pub type BRSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PXHD` reader - Protocol Exception Handling Disable
pub type PXHD_R = crate::BitReader;
///Field `PXHD` writer - Protocol Exception Handling Disable
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EFBI` reader - Edge Filtering during Bus Integration
pub type EFBI_R = crate::BitReader;
///Field `EFBI` writer - Edge Filtering during Bus Integration
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXP` reader - TXP
pub type TXP_R = crate::BitReader;
///Field `TXP` writer - TXP
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NISO` reader - Non ISO Operation
pub type NISO_R = crate::BitReader;
///Field `NISO` writer - Non ISO Operation
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configuration Change Enable
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ASM Restricted Operation Mode
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clock Stop Acknowledge
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clock Stop Request
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bus Monitoring Mode
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Disable Automatic Retransmission
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Test Mode Enable
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FD Operation Enable
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FDCAN Bit Rate Switching
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Protocol Exception Handling Disable
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Edge Filtering during Bus Integration
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TXP
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Non ISO Operation
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
impl W {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<CCCRrs> {
        INIT_W::new(self, 0)
    }
    ///Bit 1 - Configuration Change Enable
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W<CCCRrs> {
        CCE_W::new(self, 1)
    }
    ///Bit 2 - ASM Restricted Operation Mode
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W<CCCRrs> {
        ASM_W::new(self, 2)
    }
    ///Bit 3 - Clock Stop Acknowledge
    #[inline(always)]
    pub fn csa(&mut self) -> CSA_W<CCCRrs> {
        CSA_W::new(self, 3)
    }
    ///Bit 4 - Clock Stop Request
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<CCCRrs> {
        CSR_W::new(self, 4)
    }
    ///Bit 5 - Bus Monitoring Mode
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<CCCRrs> {
        MON_W::new(self, 5)
    }
    ///Bit 6 - Disable Automatic Retransmission
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<CCCRrs> {
        DAR_W::new(self, 6)
    }
    ///Bit 7 - Test Mode Enable
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<CCCRrs> {
        TEST_W::new(self, 7)
    }
    ///Bit 8 - FD Operation Enable
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W<CCCRrs> {
        FDOE_W::new(self, 8)
    }
    ///Bit 9 - FDCAN Bit Rate Switching
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W<CCCRrs> {
        BRSE_W::new(self, 9)
    }
    ///Bit 12 - Protocol Exception Handling Disable
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W<CCCRrs> {
        PXHD_W::new(self, 12)
    }
    ///Bit 13 - Edge Filtering during Bus Integration
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W<CCCRrs> {
        EFBI_W::new(self, 13)
    }
    ///Bit 14 - TXP
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<CCCRrs> {
        TXP_W::new(self, 14)
    }
    ///Bit 15 - Non ISO Operation
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W<CCCRrs> {
        NISO_W::new(self, 15)
    }
}
/**FDCAN CC Control Register

You can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:CCCR)*/
pub struct CCCRrs;
impl crate::RegisterSpec for CCCRrs {
    type Ux = u32;
}
///`read()` method returns [`cccr::R`](R) reader structure
impl crate::Readable for CCCRrs {}
///`write(|w| ..)` method takes [`cccr::W`](W) writer structure
impl crate::Writable for CCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCR to value 0x01
impl crate::Resettable for CCCRrs {
    const RESET_VALUE: u32 = 0x01;
}
