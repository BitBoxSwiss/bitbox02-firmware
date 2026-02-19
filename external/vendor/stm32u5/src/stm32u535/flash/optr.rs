///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOR_LEV` reader - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `nRST_STOP` reader - Reset generation in Stop mode
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - Reset generation in Stop mode
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - Reset generation in Standby mode
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - Reset generation in Standby mode
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_SHDW` reader - Reset generation in Shutdown mode
pub type N_RST_SHDW_R = crate::BitReader;
///Field `nRST_SHDW` writer - Reset generation in Shutdown mode
pub type N_RST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1345_RST` reader - SRAM1, SRAM4 and SRAM5 erase upon system reset
pub type SRAM1345_RST_R = crate::BitReader;
///Field `SRAM1345_RST` writer - SRAM1, SRAM4 and SRAM5 erase upon system reset
pub type SRAM1345_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - Independent watchdog selection
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK` reader - Swap banks
pub type SWAP_BANK_R = crate::BitReader;
///Field `SWAP_BANK` writer - Swap banks
pub type SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DUALBANK` reader - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
pub type DUALBANK_R = crate::BitReader;
///Field `DUALBANK` writer - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
pub type DUALBANK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction enable
pub type BKPRAM_ECC_R = crate::BitReader;
///Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction enable
pub type BKPRAM_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction enable
pub type SRAM2_ECC_R = crate::BitReader;
///Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction enable
pub type SRAM2_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_RST` reader - SRAM2 erase when system reset
pub type SRAM2_RST_R = crate::BitReader;
///Field `SRAM2_RST` writer - SRAM2 erase when system reset
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nSWBOOT0` reader - Software BOOT0
pub type N_SWBOOT0_R = crate::BitReader;
///Field `nSWBOOT0` writer - Software BOOT0
pub type N_SWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader;
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA15_PUPEN` reader - PA15 pull-up enable
pub type PA15_PUPEN_R = crate::BitReader;
///Field `PA15_PUPEN` writer - PA15 pull-up enable
pub type PA15_PUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
pub type IO_VDD_HSLV_R = crate::BitReader;
///Field `IO_VDD_HSLV` writer - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
pub type IO_VDD_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO_VDDIO2_HSLV` reader - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
pub type IO_VDDIO2_HSLV_R = crate::BitReader;
///Field `IO_VDDIO2_HSLV` writer - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
pub type IO_VDDIO2_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN` reader - Global TrustZone security enable
pub type TZEN_R = crate::BitReader;
///Field `TZEN` writer - Global TrustZone security enable
pub type TZEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - Reset generation in Stop mode
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reset generation in Standby mode
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Reset generation in Shutdown mode
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAM1, SRAM4 and SRAM5 erase upon system reset
    #[inline(always)]
    pub fn sram1345_rst(&self) -> SRAM1345_RST_R {
        SRAM1345_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Swap banks
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
    #[inline(always)]
    pub fn dualbank(&self) -> DUALBANK_R {
        DUALBANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Backup RAM ECC detection and correction enable
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - SRAM2 ECC detection and correction enable
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PA15 pull-up enable
    #[inline(always)]
    pub fn pa15_pupen(&self) -> PA15_PUPEN_R {
        PA15_PUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Global TrustZone security enable
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("bor_lev", &self.bor_lev())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("n_rst_shdw", &self.n_rst_shdw())
            .field("sram1345_rst", &self.sram1345_rst())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("swap_bank", &self.swap_bank())
            .field("dualbank", &self.dualbank())
            .field("bkpram_ecc", &self.bkpram_ecc())
            .field("sram2_ecc", &self.sram2_ecc())
            .field("sram2_rst", &self.sram2_rst())
            .field("n_swboot0", &self.n_swboot0())
            .field("n_boot0", &self.n_boot0())
            .field("pa15_pupen", &self.pa15_pupen())
            .field("io_vdd_hslv", &self.io_vdd_hslv())
            .field("io_vddio2_hslv", &self.io_vddio2_hslv())
            .field("tzen", &self.tzen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details.
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bits 8:10 - BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset.
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    ///Bit 12 - Reset generation in Stop mode
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTRrs> {
        N_RST_STOP_W::new(self, 12)
    }
    ///Bit 13 - Reset generation in Standby mode
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTRrs> {
        N_RST_STDBY_W::new(self, 13)
    }
    ///Bit 14 - Reset generation in Shutdown mode
    #[inline(always)]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<OPTRrs> {
        N_RST_SHDW_W::new(self, 14)
    }
    ///Bit 15 - SRAM1, SRAM4 and SRAM5 erase upon system reset
    #[inline(always)]
    pub fn sram1345_rst(&mut self) -> SRAM1345_RST_W<OPTRrs> {
        SRAM1345_RST_W::new(self, 15)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 20 - Swap banks
    #[inline(always)]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<OPTRrs> {
        SWAP_BANK_W::new(self, 20)
    }
    ///Bit 21 - Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices
    #[inline(always)]
    pub fn dualbank(&mut self) -> DUALBANK_W<OPTRrs> {
        DUALBANK_W::new(self, 21)
    }
    ///Bit 22 - Backup RAM ECC detection and correction enable
    #[inline(always)]
    pub fn bkpram_ecc(&mut self) -> BKPRAM_ECC_W<OPTRrs> {
        BKPRAM_ECC_W::new(self, 22)
    }
    ///Bit 24 - SRAM2 ECC detection and correction enable
    #[inline(always)]
    pub fn sram2_ecc(&mut self) -> SRAM2_ECC_W<OPTRrs> {
        SRAM2_ECC_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<OPTRrs> {
        N_SWBOOT0_W::new(self, 26)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<OPTRrs> {
        N_BOOT0_W::new(self, 27)
    }
    ///Bit 28 - PA15 pull-up enable
    #[inline(always)]
    pub fn pa15_pupen(&mut self) -> PA15_PUPEN_W<OPTRrs> {
        PA15_PUPEN_W::new(self, 28)
    }
    ///Bit 29 - High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V
    #[inline(always)]
    pub fn io_vdd_hslv(&mut self) -> IO_VDD_HSLV_W<OPTRrs> {
        IO_VDD_HSLV_W::new(self, 29)
    }
    ///Bit 30 - High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V.
    #[inline(always)]
    pub fn io_vddio2_hslv(&mut self) -> IO_VDDIO2_HSLV_W<OPTRrs> {
        IO_VDDIO2_HSLV_W::new(self, 30)
    }
    ///Bit 31 - Global TrustZone security enable
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W<OPTRrs> {
        TZEN_W::new(self, 31)
    }
}
/**FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0
impl crate::Resettable for OPTRrs {}
