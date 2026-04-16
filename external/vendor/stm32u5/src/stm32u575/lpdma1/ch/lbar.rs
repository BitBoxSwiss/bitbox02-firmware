///Register `LBAR` reader
pub type R = crate::R<LBARrs>;
///Register `LBAR` writer
pub type W = crate::W<LBARrs>;
///Field `LBA` reader - linked-list base address of LPDMA channel x
pub type LBA_R = crate::FieldReader<u16>;
///Field `LBA` writer - linked-list base address of LPDMA channel x
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - linked-list base address of LPDMA channel x
    #[inline(always)]
    pub fn lba(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LBAR").field("lba", &self.lba()).finish()
    }
}
impl W {
    ///Bits 16:31 - linked-list base address of LPDMA channel x
    #[inline(always)]
    pub fn lba(&mut self) -> LBA_W<LBARrs> {
        LBA_W::new(self, 16)
    }
}
/**LPDMA channel 0 linked-list base address register

You can [`read`](crate::Reg::read) this register and get [`lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LBARrs;
impl crate::RegisterSpec for LBARrs {
    type Ux = u32;
}
///`read()` method returns [`lbar::R`](R) reader structure
impl crate::Readable for LBARrs {}
///`write(|w| ..)` method takes [`lbar::W`](W) writer structure
impl crate::Writable for LBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LBAR to value 0
impl crate::Resettable for LBARrs {}
