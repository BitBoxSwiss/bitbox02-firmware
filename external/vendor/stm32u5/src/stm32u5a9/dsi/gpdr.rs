///Register `GPDR` reader
pub type R = crate::R<GPDRrs>;
///Register `GPDR` writer
pub type W = crate::W<GPDRrs>;
///Field `DATA1` reader - Payload byte 1 This field indicates the byte 1 of the packet payload.
pub type DATA1_R = crate::FieldReader;
///Field `DATA1` writer - Payload byte 1 This field indicates the byte 1 of the packet payload.
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA2` reader - Payload byte 2 This field indicates the byte 2 of the packet payload.
pub type DATA2_R = crate::FieldReader;
///Field `DATA2` writer - Payload byte 2 This field indicates the byte 2 of the packet payload.
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA3` reader - Payload byte 3 This field indicates the byte 3 of the packet payload.
pub type DATA3_R = crate::FieldReader;
///Field `DATA3` writer - Payload byte 3 This field indicates the byte 3 of the packet payload.
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA4` reader - Payload byte 4 This field indicates the byte 4 of the packet payload.
pub type DATA4_R = crate::FieldReader;
///Field `DATA4` writer - Payload byte 4 This field indicates the byte 4 of the packet payload.
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Payload byte 1 This field indicates the byte 1 of the packet payload.
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Payload byte 2 This field indicates the byte 2 of the packet payload.
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Payload byte 3 This field indicates the byte 3 of the packet payload.
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Payload byte 4 This field indicates the byte 4 of the packet payload.
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPDR")
            .field("data1", &self.data1())
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .field("data4", &self.data4())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Payload byte 1 This field indicates the byte 1 of the packet payload.
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<GPDRrs> {
        DATA1_W::new(self, 0)
    }
    ///Bits 8:15 - Payload byte 2 This field indicates the byte 2 of the packet payload.
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<GPDRrs> {
        DATA2_W::new(self, 8)
    }
    ///Bits 16:23 - Payload byte 3 This field indicates the byte 3 of the packet payload.
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<GPDRrs> {
        DATA3_W::new(self, 16)
    }
    ///Bits 24:31 - Payload byte 4 This field indicates the byte 4 of the packet payload.
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W<GPDRrs> {
        DATA4_W::new(self, 24)
    }
}
/**DSI Host generic payload data register

You can [`read`](crate::Reg::read) this register and get [`gpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:GPDR)*/
pub struct GPDRrs;
impl crate::RegisterSpec for GPDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpdr::R`](R) reader structure
impl crate::Readable for GPDRrs {}
///`write(|w| ..)` method takes [`gpdr::W`](W) writer structure
impl crate::Writable for GPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPDR to value 0
impl crate::Resettable for GPDRrs {}
