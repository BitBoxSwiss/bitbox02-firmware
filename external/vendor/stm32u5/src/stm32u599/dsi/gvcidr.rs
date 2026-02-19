///Register `GVCIDR` reader
pub type R = crate::R<GVCIDRrs>;
///Register `GVCIDR` writer
pub type W = crate::W<GVCIDRrs>;
///Field `VCIDRX` reader - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification.
pub type VCIDRX_R = crate::FieldReader;
///Field `VCIDRX` writer - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification.
pub type VCIDRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VCIDTX` reader - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted.
pub type VCIDTX_R = crate::FieldReader;
///Field `VCIDTX` writer - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted.
pub type VCIDTX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification.
    #[inline(always)]
    pub fn vcidrx(&self) -> VCIDRX_R {
        VCIDRX_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:17 - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted.
    #[inline(always)]
    pub fn vcidtx(&self) -> VCIDTX_R {
        VCIDTX_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GVCIDR")
            .field("vcidrx", &self.vcidrx())
            .field("vcidtx", &self.vcidtx())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification.
    #[inline(always)]
    pub fn vcidrx(&mut self) -> VCIDRX_W<GVCIDRrs> {
        VCIDRX_W::new(self, 0)
    }
    ///Bits 16:17 - Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted.
    #[inline(always)]
    pub fn vcidtx(&mut self) -> VCIDTX_W<GVCIDRrs> {
        VCIDTX_W::new(self, 16)
    }
}
/**DSI Host generic VCID register

You can [`read`](crate::Reg::read) this register and get [`gvcidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gvcidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:GVCIDR)*/
pub struct GVCIDRrs;
impl crate::RegisterSpec for GVCIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gvcidr::R`](R) reader structure
impl crate::Readable for GVCIDRrs {}
///`write(|w| ..)` method takes [`gvcidr::W`](W) writer structure
impl crate::Writable for GVCIDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GVCIDR to value 0
impl crate::Resettable for GVCIDRrs {}
