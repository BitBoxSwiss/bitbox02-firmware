///Register `P1CR` reader
pub type R = crate::R<P1CRrs>;
///Register `P1CR` writer
pub type W = crate::W<P1CRrs>;
///Field `CLKEN` reader - CLKEN
pub type CLKEN_R = crate::BitReader;
///Field `CLKEN` writer - CLKEN
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKSRC` reader - CLKSRC
pub type CLKSRC_R = crate::BitReader;
///Field `CLKSRC` writer - CLKSRC
pub type CLKSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSEN` reader - DQSEN
pub type DQSEN_R = crate::BitReader;
///Field `DQSEN` writer - DQSEN
pub type DQSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSSRC` reader - DQSSRC
pub type DQSSRC_R = crate::BitReader;
///Field `DQSSRC` writer - DQSSRC
pub type DQSSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NCSEN` reader - NCSEN
pub type NCSEN_R = crate::BitReader;
///Field `NCSEN` writer - NCSEN
pub type NCSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NCSSRC` reader - NCSSRC
pub type NCSSRC_R = crate::BitReader;
///Field `NCSSRC` writer - NCSSRC
pub type NCSSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOLEN` reader - IOLEN
pub type IOLEN_R = crate::BitReader;
///Field `IOLEN` writer - IOLEN
pub type IOLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOLSRC` reader - IOLSRC
pub type IOLSRC_R = crate::FieldReader;
///Field `IOLSRC` writer - IOLSRC
pub type IOLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOHEN` reader - IOHEN
pub type IOHEN_R = crate::BitReader;
///Field `IOHEN` writer - IOHEN
pub type IOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOHSRC` reader - IOHSR
pub type IOHSRC_R = crate::FieldReader;
///Field `IOHSRC` writer - IOHSR
pub type IOHSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - CLKEN
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CLKSRC
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - DQSEN
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DQSSRC
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - NCSEN
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NCSSRC
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - IOLEN
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - IOLSRC
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 24 - IOHEN
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - IOHSR
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CR")
            .field("iohsrc", &self.iohsrc())
            .field("iohen", &self.iohen())
            .field("iolsrc", &self.iolsrc())
            .field("iolen", &self.iolen())
            .field("ncssrc", &self.ncssrc())
            .field("ncsen", &self.ncsen())
            .field("dqssrc", &self.dqssrc())
            .field("dqsen", &self.dqsen())
            .field("clksrc", &self.clksrc())
            .field("clken", &self.clken())
            .finish()
    }
}
impl W {
    ///Bit 0 - CLKEN
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<P1CRrs> {
        CLKEN_W::new(self, 0)
    }
    ///Bit 1 - CLKSRC
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W<P1CRrs> {
        CLKSRC_W::new(self, 1)
    }
    ///Bit 4 - DQSEN
    #[inline(always)]
    pub fn dqsen(&mut self) -> DQSEN_W<P1CRrs> {
        DQSEN_W::new(self, 4)
    }
    ///Bit 5 - DQSSRC
    #[inline(always)]
    pub fn dqssrc(&mut self) -> DQSSRC_W<P1CRrs> {
        DQSSRC_W::new(self, 5)
    }
    ///Bit 8 - NCSEN
    #[inline(always)]
    pub fn ncsen(&mut self) -> NCSEN_W<P1CRrs> {
        NCSEN_W::new(self, 8)
    }
    ///Bit 9 - NCSSRC
    #[inline(always)]
    pub fn ncssrc(&mut self) -> NCSSRC_W<P1CRrs> {
        NCSSRC_W::new(self, 9)
    }
    ///Bit 16 - IOLEN
    #[inline(always)]
    pub fn iolen(&mut self) -> IOLEN_W<P1CRrs> {
        IOLEN_W::new(self, 16)
    }
    ///Bits 17:18 - IOLSRC
    #[inline(always)]
    pub fn iolsrc(&mut self) -> IOLSRC_W<P1CRrs> {
        IOLSRC_W::new(self, 17)
    }
    ///Bit 24 - IOHEN
    #[inline(always)]
    pub fn iohen(&mut self) -> IOHEN_W<P1CRrs> {
        IOHEN_W::new(self, 24)
    }
    ///Bits 25:26 - IOHSR
    #[inline(always)]
    pub fn iohsrc(&mut self) -> IOHSRC_W<P1CRrs> {
        IOHSRC_W::new(self, 25)
    }
}
/**OCTOSPI I/O manager Port 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OCTOSPIM:P1CR)*/
pub struct P1CRrs;
impl crate::RegisterSpec for P1CRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cr::R`](R) reader structure
impl crate::Readable for P1CRrs {}
///`write(|w| ..)` method takes [`p1cr::W`](W) writer structure
impl crate::Writable for P1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CR to value 0x0301_0111
impl crate::Resettable for P1CRrs {
    const RESET_VALUE: u32 = 0x0301_0111;
}
