///Register `GRXSTSR_HOST` reader
pub type R = crate::R<GRXSTSR_HOSTrs>;
///Field `CHNUM` reader - CHNUM
pub type CHNUM_R = crate::FieldReader;
///Field `BCNT` reader - BCNT
pub type BCNT_R = crate::FieldReader<u16>;
///Field `DPID` reader - DPID
pub type DPID_R = crate::FieldReader;
///Field `PKTSTS` reader - PKTSTS
pub type PKTSTS_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CHNUM
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - BCNT
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - DPID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - PKTSTS
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR_HOST")
            .field("chnum", &self.chnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .finish()
    }
}
/**This description is for register GRXSTSR in Host mode

You can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRXSTSR_HOST)*/
pub struct GRXSTSR_HOSTrs;
impl crate::RegisterSpec for GRXSTSR_HOSTrs {
    type Ux = u32;
}
///`read()` method returns [`grxstsr_host::R`](R) reader structure
impl crate::Readable for GRXSTSR_HOSTrs {}
///`reset()` method sets GRXSTSR_HOST to value 0
impl crate::Resettable for GRXSTSR_HOSTrs {}
