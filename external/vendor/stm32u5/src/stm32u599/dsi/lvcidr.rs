///Register `LVCIDR` reader
pub type R = crate::R<LVCIDRrs>;
///Register `LVCIDR` writer
pub type W = crate::W<LVCIDRrs>;
///Field `VCID` reader - Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic.
pub type VCID_R = crate::FieldReader;
///Field `VCID` writer - Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic.
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic.
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVCIDR")
            .field("vcid", &self.vcid())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic.
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W<LVCIDRrs> {
        VCID_W::new(self, 0)
    }
}
/**DSI Host LTDC VCID register

You can [`read`](crate::Reg::read) this register and get [`lvcidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:LVCIDR)*/
pub struct LVCIDRrs;
impl crate::RegisterSpec for LVCIDRrs {
    type Ux = u32;
}
///`read()` method returns [`lvcidr::R`](R) reader structure
impl crate::Readable for LVCIDRrs {}
///`write(|w| ..)` method takes [`lvcidr::W`](W) writer structure
impl crate::Writable for LVCIDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVCIDR to value 0
impl crate::Resettable for LVCIDRrs {}
