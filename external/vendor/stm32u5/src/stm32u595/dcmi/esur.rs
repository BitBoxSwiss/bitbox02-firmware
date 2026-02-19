///Register `ESUR` reader
pub type R = crate::R<ESURrs>;
///Register `ESUR` writer
pub type W = crate::W<ESURrs>;
///Field `FSU` reader - Frame start delimiter unmask
pub type FSU_R = crate::FieldReader;
///Field `FSU` writer - Frame start delimiter unmask
pub type FSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LSU` reader - Line start delimiter unmask
pub type LSU_R = crate::FieldReader;
///Field `LSU` writer - Line start delimiter unmask
pub type LSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LEU` reader - Line end delimiter unmask
pub type LEU_R = crate::FieldReader;
///Field `LEU` writer - Line end delimiter unmask
pub type LEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FEU` reader - Frame end delimiter unmask
pub type FEU_R = crate::FieldReader;
///Field `FEU` writer - Frame end delimiter unmask
pub type FEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Frame start delimiter unmask
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Line start delimiter unmask
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Line end delimiter unmask
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Frame end delimiter unmask
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESUR")
            .field("feu", &self.feu())
            .field("leu", &self.leu())
            .field("lsu", &self.lsu())
            .field("fsu", &self.fsu())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Frame start delimiter unmask
    #[inline(always)]
    pub fn fsu(&mut self) -> FSU_W<ESURrs> {
        FSU_W::new(self, 0)
    }
    ///Bits 8:15 - Line start delimiter unmask
    #[inline(always)]
    pub fn lsu(&mut self) -> LSU_W<ESURrs> {
        LSU_W::new(self, 8)
    }
    ///Bits 16:23 - Line end delimiter unmask
    #[inline(always)]
    pub fn leu(&mut self) -> LEU_W<ESURrs> {
        LEU_W::new(self, 16)
    }
    ///Bits 24:31 - Frame end delimiter unmask
    #[inline(always)]
    pub fn feu(&mut self) -> FEU_W<ESURrs> {
        FEU_W::new(self, 24)
    }
}
/**embedded synchronization unmask register

You can [`read`](crate::Reg::read) this register and get [`esur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DCMI:ESUR)*/
pub struct ESURrs;
impl crate::RegisterSpec for ESURrs {
    type Ux = u32;
}
///`read()` method returns [`esur::R`](R) reader structure
impl crate::Readable for ESURrs {}
///`write(|w| ..)` method takes [`esur::W`](W) writer structure
impl crate::Writable for ESURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESUR to value 0
impl crate::Resettable for ESURrs {}
