///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader;
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader;
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader;
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader;
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTSE` reader - RTS enable
pub type RTSE_R = crate::BitReader;
///Field `RTSE` writer - RTS enable
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSE` reader - CTS enable
pub type CTSE_R = crate::BitReader;
///Field `CTSE` writer - CTS enable
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSIE` reader - CTS interrupt enable
pub type CTSIE_R = crate::BitReader;
///Field `CTSIE` writer - CTS interrupt enable
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRDIS` reader - Overrun Disable
pub type OVRDIS_R = crate::BitReader;
///Field `OVRDIS` writer - Overrun Disable
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRE` reader - DMA Disable on Reception Error
pub type DDRE_R = crate::BitReader;
///Field `DDRE` writer - DMA Disable on Reception Error
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEM` reader - Driver enable mode
pub type DEM_R = crate::BitReader;
///Field `DEM` writer - Driver enable mode
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEP` reader - Driver enable polarity selection
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - Driver enable polarity selection
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFTIE` reader - TXFTIE
pub type TXFTIE_R = crate::BitReader;
///Field `TXFTIE` writer - TXFTIE
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFTCFG` reader - RXFTCFG
pub type RXFTCFG_R = crate::FieldReader;
///Field `RXFTCFG` writer - RXFTCFG
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXFTIE` reader - RXFTIE
pub type RXFTIE_R = crate::BitReader;
///Field `RXFTIE` writer - RXFTIE
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFTCFG` reader - TXFTCFG
pub type TXFTCFG_R = crate::FieldReader;
///Field `TXFTCFG` writer - TXFTCFG
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 23 - TXFTIE
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:27 - RXFTCFG
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFTIE
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFTCFG
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("txftcfg", &self.txftcfg())
            .field("rxftie", &self.rxftie())
            .field("rxftcfg", &self.rxftcfg())
            .field("txftie", &self.txftie())
            .field("dep", &self.dep())
            .field("dem", &self.dem())
            .field("ddre", &self.ddre())
            .field("ovrdis", &self.ovrdis())
            .field("ctsie", &self.ctsie())
            .field("ctse", &self.ctse())
            .field("rtse", &self.rtse())
            .field("dmat", &self.dmat())
            .field("dmar", &self.dmar())
            .field("hdsel", &self.hdsel())
            .field("eie", &self.eie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bit 23 - TXFTIE
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bits 25:27 - RXFTCFG
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFTIE
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFTCFG
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPUART1:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
