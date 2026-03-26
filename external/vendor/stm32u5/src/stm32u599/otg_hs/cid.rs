///Register `CID` reader
pub type R = crate::R<CIDrs>;
///Register `CID` writer
pub type W = crate::W<CIDrs>;
///Field `PRODUCT_ID` reader - PRODUCT_ID
pub type PRODUCT_ID_R = crate::FieldReader<u32>;
///Field `PRODUCT_ID` writer - PRODUCT_ID
pub type PRODUCT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PRODUCT_ID
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID")
            .field("product_id", &self.product_id())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PRODUCT_ID
    #[inline(always)]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<CIDrs> {
        PRODUCT_ID_W::new(self, 0)
    }
}
/**This is a register containing the Product ID as reset value.

You can [`read`](crate::Reg::read) this register and get [`cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:CID)*/
pub struct CIDrs;
impl crate::RegisterSpec for CIDrs {
    type Ux = u32;
}
///`read()` method returns [`cid::R`](R) reader structure
impl crate::Readable for CIDrs {}
///`write(|w| ..)` method takes [`cid::W`](W) writer structure
impl crate::Writable for CIDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CID to value 0x3100
impl crate::Resettable for CIDrs {
    const RESET_VALUE: u32 = 0x3100;
}
