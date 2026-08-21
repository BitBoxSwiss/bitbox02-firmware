///Register `HCTSIZ15` reader
pub type R = crate::R<HCTSIZ15rs>;
///Register `HCTSIZ15` writer
pub type W = crate::W<HCTSIZ15rs>;
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
        f.debug_struct("HCTSIZ15")
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
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<HCTSIZ15rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - PKTCNT
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<HCTSIZ15rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - DPID
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W<HCTSIZ15rs> {
        DPID_W::new(self, 29)
    }
    ///Bit 31 - DOPNG
    #[inline(always)]
    pub fn dopng(&mut self) -> DOPNG_W<HCTSIZ15rs> {
        DOPNG_W::new(self, 31)
    }
}
/**OTG host channel 15 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:HCTSIZ15)*/
pub struct HCTSIZ15rs;
impl crate::RegisterSpec for HCTSIZ15rs {
    type Ux = u32;
}
///`read()` method returns [`hctsiz15::R`](R) reader structure
impl crate::Readable for HCTSIZ15rs {}
///`write(|w| ..)` method takes [`hctsiz15::W`](W) writer structure
impl crate::Writable for HCTSIZ15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCTSIZ15 to value 0
impl crate::Resettable for HCTSIZ15rs {}
