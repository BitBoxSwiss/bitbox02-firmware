///Register `B3CR` reader
pub type R = crate::R<B3CRrs>;
///Register `B3CR` writer
pub type W = crate::W<B3CRrs>;
///Field `PBO` reader - Physical buffer offset Offset of the physical buffer.
pub type PBO_R = crate::FieldReader<u32>;
///Field `PBO` writer - Physical buffer offset Offset of the physical buffer.
pub type PBO_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PBBA` reader - Physical buffer base address Base address MSB of the physical buffer.
pub type PBBA_R = crate::FieldReader<u16>;
///Field `PBBA` writer - Physical buffer base address Base address MSB of the physical buffer.
pub type PBBA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 4:22 - Physical buffer offset Offset of the physical buffer.
    #[inline(always)]
    pub fn pbo(&self) -> PBO_R {
        PBO_R::new((self.bits >> 4) & 0x0007_ffff)
    }
    ///Bits 23:31 - Physical buffer base address Base address MSB of the physical buffer.
    #[inline(always)]
    pub fn pbba(&self) -> PBBA_R {
        PBBA_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B3CR")
            .field("pbo", &self.pbo())
            .field("pbba", &self.pbba())
            .finish()
    }
}
impl W {
    ///Bits 4:22 - Physical buffer offset Offset of the physical buffer.
    #[inline(always)]
    pub fn pbo(&mut self) -> PBO_W<B3CRrs> {
        PBO_W::new(self, 4)
    }
    ///Bits 23:31 - Physical buffer base address Base address MSB of the physical buffer.
    #[inline(always)]
    pub fn pbba(&mut self) -> PBBA_W<B3CRrs> {
        PBBA_W::new(self, 23)
    }
}
/**GFXMMU buffer 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`b3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GFXMMU:B3CR)*/
pub struct B3CRrs;
impl crate::RegisterSpec for B3CRrs {
    type Ux = u32;
}
///`read()` method returns [`b3cr::R`](R) reader structure
impl crate::Readable for B3CRrs {}
///`write(|w| ..)` method takes [`b3cr::W`](W) writer structure
impl crate::Writable for B3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets B3CR to value 0
impl crate::Resettable for B3CRrs {}
