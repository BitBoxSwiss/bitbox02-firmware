///Register `VNPCR` reader
pub type R = crate::R<VNPCRrs>;
///Register `VNPCR` writer
pub type W = crate::W<VNPCRrs>;
///Field `NPSIZE` reader - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
pub type NPSIZE_R = crate::FieldReader<u16>;
///Field `NPSIZE` writer - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
pub type NPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VNPCR")
            .field("npsize", &self.npsize())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets.
    #[inline(always)]
    pub fn npsize(&mut self) -> NPSIZE_W<VNPCRrs> {
        NPSIZE_W::new(self, 0)
    }
}
/**DSI Host video null packet configuration register

You can [`read`](crate::Reg::read) this register and get [`vnpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vnpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:VNPCR)*/
pub struct VNPCRrs;
impl crate::RegisterSpec for VNPCRrs {
    type Ux = u32;
}
///`read()` method returns [`vnpcr::R`](R) reader structure
impl crate::Readable for VNPCRrs {}
///`write(|w| ..)` method takes [`vnpcr::W`](W) writer structure
impl crate::Writable for VNPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VNPCR to value 0
impl crate::Resettable for VNPCRrs {}
