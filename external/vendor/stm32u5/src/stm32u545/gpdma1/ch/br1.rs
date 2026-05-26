///Register `BR1` reader
pub type R = crate::R<BR1rs>;
///Register `BR1` writer
pub type W = crate::W<BR1rs>;
///Field `BNDT` reader - block number of data bytes to transfer from the source
pub type BNDT_R = crate::FieldReader<u16>;
///Field `BNDT` writer - block number of data bytes to transfer from the source
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BR1").field("bndt", &self.bndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<BR1rs> {
        BNDT_W::new(self, 0)
    }
}
/**GPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BR1rs;
impl crate::RegisterSpec for BR1rs {
    type Ux = u32;
}
///`read()` method returns [`br1::R`](R) reader structure
impl crate::Readable for BR1rs {}
///`write(|w| ..)` method takes [`br1::W`](W) writer structure
impl crate::Writable for BR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BR1 to value 0
impl crate::Resettable for BR1rs {}
