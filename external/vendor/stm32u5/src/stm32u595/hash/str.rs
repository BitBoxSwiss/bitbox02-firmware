///Register `STR` reader
pub type R = crate::R<STRrs>;
///Register `STR` writer
pub type W = crate::W<STRrs>;
///Field `NBLW` reader - Number of valid bits in the last word of the message
pub type NBLW_R = crate::FieldReader;
///Field `NBLW` writer - Number of valid bits in the last word of the message
pub type NBLW_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DCAL` writer - Digest calculation
pub type DCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Number of valid bits in the last word of the message
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR").field("nblw", &self.nblw()).finish()
    }
}
impl W {
    ///Bits 0:4 - Number of valid bits in the last word of the message
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W<STRrs> {
        NBLW_W::new(self, 0)
    }
    ///Bit 8 - Digest calculation
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W<STRrs> {
        DCAL_W::new(self, 8)
    }
}
/**start register

You can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#HASH:STR)*/
pub struct STRrs;
impl crate::RegisterSpec for STRrs {
    type Ux = u32;
}
///`read()` method returns [`str::R`](R) reader structure
impl crate::Readable for STRrs {}
///`write(|w| ..)` method takes [`str::W`](W) writer structure
impl crate::Writable for STRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STR to value 0
impl crate::Resettable for STRrs {}
