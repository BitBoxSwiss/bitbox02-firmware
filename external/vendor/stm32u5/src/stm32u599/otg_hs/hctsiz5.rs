///Register `HCTSIZ5` reader
pub type R = crate::R<HCTSIZ5rs>;
///Register `HCTSIZ5` writer
pub type W = crate::W<HCTSIZ5rs>;
///Field `XFRSIZ` reader - XFRSIZ
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - XFRSIZ
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - PKTCNT
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - PKTCNT
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DPID` reader - DPID
pub type DPID_R = crate::FieldReader;
///Field `DPID` writer - DPID
pub type DPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DOPNG` reader - DOPNG
pub type DOPNG_R = crate::BitReader;
///Field `DOPNG` writer - DOPNG
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:18 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - DPID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - DOPNG
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ5")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("dpid", &self.dpid())
            .field("dopng", &self.dopng())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - XFRSIZ
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<HCTSIZ5rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<HCTSIZ5rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - DPID
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W<HCTSIZ5rs> {
        DPID_W::new(self, 29)
    }
    ///Bit 31 - DOPNG
    #[inline(always)]
    pub fn dopng(&mut self) -> DOPNG_W<HCTSIZ5rs> {
        DOPNG_W::new(self, 31)
    }
}
/**OTG host channel 5 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:HCTSIZ5)*/
pub struct HCTSIZ5rs;
impl crate::RegisterSpec for HCTSIZ5rs {
    type Ux = u32;
}
///`read()` method returns [`hctsiz5::R`](R) reader structure
impl crate::Readable for HCTSIZ5rs {}
///`write(|w| ..)` method takes [`hctsiz5::W`](W) writer structure
impl crate::Writable for HCTSIZ5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCTSIZ5 to value 0
impl crate::Resettable for HCTSIZ5rs {}
