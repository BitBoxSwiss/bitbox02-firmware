///Register `GOTGCTL` reader
pub type R = crate::R<GOTGCTLrs>;
///Register `GOTGCTL` writer
pub type W = crate::W<GOTGCTLrs>;
///Field `SRQSCS` reader - SRQSCS
pub type SRQSCS_R = crate::BitReader;
///Field `SRQ` reader - SRQ
pub type SRQ_R = crate::BitReader;
///Field `SRQ` writer - SRQ
pub type SRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOEN` reader - VBVALOEN
pub type VBVALOEN_R = crate::BitReader;
///Field `VBVALOEN` writer - VBVALOEN
pub type VBVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOVAL` reader - VBVALOVAL
pub type VBVALOVAL_R = crate::BitReader;
///Field `VBVALOVAL` writer - VBVALOVAL
pub type VBVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVALOEN` reader - AVALOEN
pub type AVALOEN_R = crate::BitReader;
///Field `AVALOEN` writer - AVALOEN
pub type AVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVALOVAL` reader - AVALOVAL
pub type AVALOVAL_R = crate::BitReader;
///Field `AVALOVAL` writer - AVALOVAL
pub type AVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BVALOEN` reader - BVALOEN
pub type BVALOEN_R = crate::BitReader;
///Field `BVALOEN` writer - BVALOEN
pub type BVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BVALOVAL` reader - BVALOVAL
pub type BVALOVAL_R = crate::BitReader;
///Field `BVALOVAL` writer - BVALOVAL
pub type BVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNGSCS` reader - HNGSCS
pub type HNGSCS_R = crate::BitReader;
///Field `HNPRQ` reader - HNPRQ
pub type HNPRQ_R = crate::BitReader;
///Field `HNPRQ` writer - HNPRQ
pub type HNPRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSHNPEN` reader - HSHNPEN
pub type HSHNPEN_R = crate::BitReader;
///Field `HSHNPEN` writer - HSHNPEN
pub type HSHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHNPEN` reader - DHNPEN
pub type DHNPEN_R = crate::BitReader;
///Field `DHNPEN` writer - DHNPEN
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EHEN` reader - EHEN
pub type EHEN_R = crate::BitReader;
///Field `EHEN` writer - EHEN
pub type EHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSTS` reader - CIDSTS
pub type CIDSTS_R = crate::BitReader;
///Field `DBCT` reader - DBCT
pub type DBCT_R = crate::BitReader;
///Field `ASVLD` reader - ASVLD
pub type ASVLD_R = crate::BitReader;
///Field `BSVLD` reader - BSVLD
pub type BSVLD_R = crate::BitReader;
///Field `OTGVER` reader - OTGVER
pub type OTGVER_R = crate::BitReader;
///Field `OTGVER` writer - OTGVER
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CURMOD` reader - CURMOD
pub type CURMOD_R = crate::BitReader;
impl R {
    ///Bit 0 - SRQSCS
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRQ
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBVALOEN
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBVALOVAL
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AVALOEN
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AVALOVAL
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BVALOEN
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BVALOVAL
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HNGSCS
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNPRQ
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSHNPEN
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DHNPEN
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EHEN
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - CIDSTS
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DBCT
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ASVLD
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - BSVLD
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - OTGVER
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CURMOD
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGCTL")
            .field("srqscs", &self.srqscs())
            .field("srq", &self.srq())
            .field("vbvaloen", &self.vbvaloen())
            .field("vbvaloval", &self.vbvaloval())
            .field("avaloen", &self.avaloen())
            .field("avaloval", &self.avaloval())
            .field("bvaloen", &self.bvaloen())
            .field("bvaloval", &self.bvaloval())
            .field("hngscs", &self.hngscs())
            .field("hnprq", &self.hnprq())
            .field("hshnpen", &self.hshnpen())
            .field("dhnpen", &self.dhnpen())
            .field("ehen", &self.ehen())
            .field("cidsts", &self.cidsts())
            .field("dbct", &self.dbct())
            .field("asvld", &self.asvld())
            .field("bsvld", &self.bsvld())
            .field("otgver", &self.otgver())
            .field("curmod", &self.curmod())
            .finish()
    }
}
impl W {
    ///Bit 1 - SRQ
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W<GOTGCTLrs> {
        SRQ_W::new(self, 1)
    }
    ///Bit 2 - VBVALOEN
    #[inline(always)]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<GOTGCTLrs> {
        VBVALOEN_W::new(self, 2)
    }
    ///Bit 3 - VBVALOVAL
    #[inline(always)]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W<GOTGCTLrs> {
        VBVALOVAL_W::new(self, 3)
    }
    ///Bit 4 - AVALOEN
    #[inline(always)]
    pub fn avaloen(&mut self) -> AVALOEN_W<GOTGCTLrs> {
        AVALOEN_W::new(self, 4)
    }
    ///Bit 5 - AVALOVAL
    #[inline(always)]
    pub fn avaloval(&mut self) -> AVALOVAL_W<GOTGCTLrs> {
        AVALOVAL_W::new(self, 5)
    }
    ///Bit 6 - BVALOEN
    #[inline(always)]
    pub fn bvaloen(&mut self) -> BVALOEN_W<GOTGCTLrs> {
        BVALOEN_W::new(self, 6)
    }
    ///Bit 7 - BVALOVAL
    #[inline(always)]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<GOTGCTLrs> {
        BVALOVAL_W::new(self, 7)
    }
    ///Bit 9 - HNPRQ
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W<GOTGCTLrs> {
        HNPRQ_W::new(self, 9)
    }
    ///Bit 10 - HSHNPEN
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<GOTGCTLrs> {
        HSHNPEN_W::new(self, 10)
    }
    ///Bit 11 - DHNPEN
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W<GOTGCTLrs> {
        DHNPEN_W::new(self, 11)
    }
    ///Bit 12 - EHEN
    #[inline(always)]
    pub fn ehen(&mut self) -> EHEN_W<GOTGCTLrs> {
        EHEN_W::new(self, 12)
    }
    ///Bit 20 - OTGVER
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W<GOTGCTLrs> {
        OTGVER_W::new(self, 20)
    }
}
/**The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.

You can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#OTG_HS:GOTGCTL)*/
pub struct GOTGCTLrs;
impl crate::RegisterSpec for GOTGCTLrs {
    type Ux = u32;
}
///`read()` method returns [`gotgctl::R`](R) reader structure
impl crate::Readable for GOTGCTLrs {}
///`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure
impl crate::Writable for GOTGCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GOTGCTL to value 0x0001_0000
impl crate::Resettable for GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
