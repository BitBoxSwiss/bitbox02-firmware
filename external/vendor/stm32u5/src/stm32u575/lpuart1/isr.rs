///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `PE` reader - PE
pub type PE_R = crate::BitReader;
///Field `FE` reader - FE
pub type FE_R = crate::BitReader;
///Field `NE` reader - NE
pub type NE_R = crate::BitReader;
///Field `ORE` reader - ORE
pub type ORE_R = crate::BitReader;
///Field `IDLE` reader - IDLE
pub type IDLE_R = crate::BitReader;
///Field `RXFNE` reader - RXFNE
pub type RXFNE_R = crate::BitReader;
///Field `TC` reader - TC
pub type TC_R = crate::BitReader;
///Field `TXFNF` reader - TXFNF
pub type TXFNF_R = crate::BitReader;
///Field `CTSIF` reader - CTSIF
pub type CTSIF_R = crate::BitReader;
///Field `CTS` reader - CTS
pub type CTS_R = crate::BitReader;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
///Field `CMF` reader - CMF
pub type CMF_R = crate::BitReader;
///Field `SBKF` reader - SBKF
pub type SBKF_R = crate::BitReader;
///Field `RWU` reader - RWU
pub type RWU_R = crate::BitReader;
///Field `TEACK` reader - TEACK
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - REACK
pub type REACK_R = crate::BitReader;
///Field `TXFE` reader - TXFE
pub type TXFE_R = crate::BitReader;
///Field `RXFF` reader - RXFF
pub type RXFF_R = crate::BitReader;
///Field `RXFT` reader - RXFT
pub type RXFT_R = crate::BitReader;
///Field `TXFT` reader - TXFT
pub type TXFT_R = crate::BitReader;
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NE
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFNE
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFNF
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFE
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFF
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RXFT
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFT
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("txft", &self.txft())
            .field("rxft", &self.rxft())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("reack", &self.reack())
            .field("teack", &self.teack())
            .field("rwu", &self.rwu())
            .field("sbkf", &self.sbkf())
            .field("cmf", &self.cmf())
            .field("busy", &self.busy())
            .field("cts", &self.cts())
            .field("ctsif", &self.ctsif())
            .field("txfnf", &self.txfnf())
            .field("tc", &self.tc())
            .field("rxfne", &self.rxfne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("ne", &self.ne())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .finish()
    }
}
/**Interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#LPUART1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0x0080_00c0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x0080_00c0;
}
