///Register `GRSTCTL` reader
pub type R = crate::R<GRSTCTLrs>;
///Register `GRSTCTL` writer
pub type W = crate::W<GRSTCTLrs>;
///Field `CSRST` reader - CSRST
pub type CSRST_R = crate::BitReader;
///Field `PSRST` reader - PSRST
pub type PSRST_R = crate::BitReader;
///Field `PSRST` writer - PSRST
pub type PSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSRST` reader - FSRST
pub type FSRST_R = crate::BitReader;
///Field `FSRST` writer - FSRST
pub type FSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFLSH` reader - RXFFLSH
pub type RXFFLSH_R = crate::BitReader;
///Field `RXFFLSH` writer - RXFFLSH
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFFLSH` reader - TXFFLSH
pub type TXFFLSH_R = crate::BitReader;
///Field `TXFFLSH` writer - TXFFLSH
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - TXFNUM
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - TXFNUM
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `AHBIDL` reader - AHBIDL
pub type AHBIDL_R = crate::BitReader;
impl R {
    ///Bit 0 - CSRST
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PSRST
    #[inline(always)]
    pub fn psrst(&self) -> PSRST_R {
        PSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FSRST
    #[inline(always)]
    pub fn fsrst(&self) -> FSRST_R {
        FSRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RXFFLSH
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TXFFLSH
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 31 - AHBIDL
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csrst", &self.csrst())
            .field("psrst", &self.psrst())
            .field("fsrst", &self.fsrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("ahbidl", &self.ahbidl())
            .finish()
    }
}
impl W {
    ///Bit 1 - PSRST
    #[inline(always)]
    pub fn psrst(&mut self) -> PSRST_W<GRSTCTLrs> {
        PSRST_W::new(self, 1)
    }
    ///Bit 2 - FSRST
    #[inline(always)]
    pub fn fsrst(&mut self) -> FSRST_W<GRSTCTLrs> {
        FSRST_W::new(self, 2)
    }
    ///Bit 4 - RXFFLSH
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    ///Bit 5 - TXFFLSH
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    ///Bits 6:10 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
/**The application uses this register to reset various hardware features inside the core.

You can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRSTCTL)*/
pub struct GRSTCTLrs;
impl crate::RegisterSpec for GRSTCTLrs {
    type Ux = u32;
}
///`read()` method returns [`grstctl::R`](R) reader structure
impl crate::Readable for GRSTCTLrs {}
///`write(|w| ..)` method takes [`grstctl::W`](W) writer structure
impl crate::Writable for GRSTCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GRSTCTL to value 0x8000_0000
impl crate::Resettable for GRSTCTLrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
