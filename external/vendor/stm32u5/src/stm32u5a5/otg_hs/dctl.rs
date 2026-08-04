///Register `DCTL` reader
pub type R = crate::R<DCTLrs>;
///Register `DCTL` writer
pub type W = crate::W<DCTLrs>;
///Field `RWUSIG` reader - RWUSIG
pub type RWUSIG_R = crate::BitReader;
///Field `RWUSIG` writer - RWUSIG
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIS` reader - SDIS
pub type SDIS_R = crate::BitReader;
///Field `SDIS` writer - SDIS
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GINSTS` reader - GINSTS
pub type GINSTS_R = crate::BitReader;
///Field `GONSTS` reader - GONSTS
pub type GONSTS_R = crate::BitReader;
///Field `TCTL` reader - TCTL
pub type TCTL_R = crate::FieldReader;
///Field `TCTL` writer - TCTL
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SGINAK` writer - SGINAK
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGINAK` writer - CGINAK
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SGONAK` writer - SGONAK
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGONAK` writer - CGONAK
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POPRGDNE` reader - POPRGDNE
pub type POPRGDNE_R = crate::BitReader;
///Field `POPRGDNE` writer - POPRGDNE
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSBESLRJCT` reader - DSBESLRJCT
pub type DSBESLRJCT_R = crate::BitReader;
///Field `DSBESLRJCT` writer - DSBESLRJCT
pub type DSBESLRJCT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RWUSIG
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SDIS
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GINSTS
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GONSTS
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - TCTL
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 11 - POPRGDNE
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 18 - DSBESLRJCT
    #[inline(always)]
    pub fn dsbeslrjct(&self) -> DSBESLRJCT_R {
        DSBESLRJCT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rwusig", &self.rwusig())
            .field("sdis", &self.sdis())
            .field("ginsts", &self.ginsts())
            .field("gonsts", &self.gonsts())
            .field("tctl", &self.tctl())
            .field("poprgdne", &self.poprgdne())
            .field("dsbeslrjct", &self.dsbeslrjct())
            .finish()
    }
}
impl W {
    ///Bit 0 - RWUSIG
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W<DCTLrs> {
        RWUSIG_W::new(self, 0)
    }
    ///Bit 1 - SDIS
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W<DCTLrs> {
        SDIS_W::new(self, 1)
    }
    ///Bits 4:6 - TCTL
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W<DCTLrs> {
        TCTL_W::new(self, 4)
    }
    ///Bit 7 - SGINAK
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W<DCTLrs> {
        SGINAK_W::new(self, 7)
    }
    ///Bit 8 - CGINAK
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W<DCTLrs> {
        CGINAK_W::new(self, 8)
    }
    ///Bit 9 - SGONAK
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W<DCTLrs> {
        SGONAK_W::new(self, 9)
    }
    ///Bit 10 - CGONAK
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W<DCTLrs> {
        CGONAK_W::new(self, 10)
    }
    ///Bit 11 - POPRGDNE
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<DCTLrs> {
        POPRGDNE_W::new(self, 11)
    }
    ///Bit 18 - DSBESLRJCT
    #[inline(always)]
    pub fn dsbeslrjct(&mut self) -> DSBESLRJCT_W<DCTLrs> {
        DSBESLRJCT_W::new(self, 18)
    }
}
/**OTG device control register

You can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:DCTL)*/
pub struct DCTLrs;
impl crate::RegisterSpec for DCTLrs {
    type Ux = u32;
}
///`read()` method returns [`dctl::R`](R) reader structure
impl crate::Readable for DCTLrs {}
///`write(|w| ..)` method takes [`dctl::W`](W) writer structure
impl crate::Writable for DCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCTL to value 0x02
impl crate::Resettable for DCTLrs {
    const RESET_VALUE: u32 = 0x02;
}
