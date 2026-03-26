///Register `CKDIV` reader
pub type R = crate::R<CKDIVrs>;
///Register `CKDIV` writer
pub type W = crate::W<CKDIVrs>;
///Field `PDIV` reader - PDIV
pub type PDIV_R = crate::FieldReader;
///Field `PDIV` writer - PDIV
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - PDIV
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKDIV").field("pdiv", &self.pdiv()).finish()
    }
}
impl W {
    ///Bits 0:3 - PDIV
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W<CKDIVrs> {
        PDIV_W::new(self, 0)
    }
}
/**FDCAN CFG clock divider register

You can [`read`](crate::Reg::read) this register and get [`ckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#FDCAN1_RAM:CKDIV)*/
pub struct CKDIVrs;
impl crate::RegisterSpec for CKDIVrs {
    type Ux = u32;
}
///`read()` method returns [`ckdiv::R`](R) reader structure
impl crate::Readable for CKDIVrs {}
///`write(|w| ..)` method takes [`ckdiv::W`](W) writer structure
impl crate::Writable for CKDIVrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV to value 0
impl crate::Resettable for CKDIVrs {}
