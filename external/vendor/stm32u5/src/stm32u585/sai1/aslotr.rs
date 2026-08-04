///Register `ASLOTR` reader
pub type R = crate::R<ASLOTRrs>;
///Register `ASLOTR` writer
pub type W = crate::W<ASLOTRrs>;
///Field `FBOFF` reader - First bit offset
pub type FBOFF_R = crate::FieldReader;
///Field `FBOFF` writer - First bit offset
pub type FBOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SLOTSZ` reader - Slot size
pub type SLOTSZ_R = crate::FieldReader;
///Field `SLOTSZ` writer - Slot size
pub type SLOTSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NBSLOT` reader - Number of slots in an audio frame
pub type NBSLOT_R = crate::FieldReader;
///Field `NBSLOT` writer - Number of slots in an audio frame
pub type NBSLOT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SLOTEN` reader - Slot enable
pub type SLOTEN_R = crate::FieldReader<u16>;
///Field `SLOTEN` writer - Slot enable
pub type SLOTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Slot enable
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASLOTR")
            .field("sloten", &self.sloten())
            .field("nbslot", &self.nbslot())
            .field("slotsz", &self.slotsz())
            .field("fboff", &self.fboff())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    pub fn fboff(&mut self) -> FBOFF_W<ASLOTRrs> {
        FBOFF_W::new(self, 0)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    pub fn slotsz(&mut self) -> SLOTSZ_W<ASLOTRrs> {
        SLOTSZ_W::new(self, 6)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    pub fn nbslot(&mut self) -> NBSLOT_W<ASLOTRrs> {
        NBSLOT_W::new(self, 8)
    }
    ///Bits 16:31 - Slot enable
    #[inline(always)]
    pub fn sloten(&mut self) -> SLOTEN_W<ASLOTRrs> {
        SLOTEN_W::new(self, 16)
    }
}
/**A Slot register

You can [`read`](crate::Reg::read) this register and get [`aslotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aslotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SAI1:ASLOTR)*/
pub struct ASLOTRrs;
impl crate::RegisterSpec for ASLOTRrs {
    type Ux = u32;
}
///`read()` method returns [`aslotr::R`](R) reader structure
impl crate::Readable for ASLOTRrs {}
///`write(|w| ..)` method takes [`aslotr::W`](W) writer structure
impl crate::Writable for ASLOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASLOTR to value 0
impl crate::Resettable for ASLOTRrs {}
