///Register `FCR` reader
pub type R = crate::R<FCRrs>;
///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `CB0OF` reader - Clear buffer 0 overflow flag Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register.
pub type CB0OF_R = crate::BitReader;
///Field `CB0OF` writer - Clear buffer 0 overflow flag Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register.
pub type CB0OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CB1OF` reader - Clear buffer 1 overflow flag Writing 1 clears the buffer 1 overflow flag in the GFXMMU_SR register.
pub type CB1OF_R = crate::BitReader;
///Field `CB1OF` writer - Clear buffer 1 overflow flag Writing 1 clears the buffer 1 overflow flag in the GFXMMU_SR register.
pub type CB1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CB2OF` reader - Clear buffer 2 overflow flag Writing 1 clears the buffer 2 overflow flag in the GFXMMU_SR register.
pub type CB2OF_R = crate::BitReader;
///Field `CB2OF` writer - Clear buffer 2 overflow flag Writing 1 clears the buffer 2 overflow flag in the GFXMMU_SR register.
pub type CB2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CB3OF` reader - Clear buffer 3 overflow flag Writing 1 clears the buffer 3 overflow flag in the GFXMMU_SR register.
pub type CB3OF_R = crate::BitReader;
///Field `CB3OF` writer - Clear buffer 3 overflow flag Writing 1 clears the buffer 3 overflow flag in the GFXMMU_SR register.
pub type CB3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAMEF` reader - Clear AHB master error flag Writing 1 clears the AHB master error flag in the GFXMMU_SR register.
pub type CAMEF_R = crate::BitReader;
///Field `CAMEF` writer - Clear AHB master error flag Writing 1 clears the AHB master error flag in the GFXMMU_SR register.
pub type CAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear buffer 0 overflow flag Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb0of(&self) -> CB0OF_R {
        CB0OF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear buffer 1 overflow flag Writing 1 clears the buffer 1 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb1of(&self) -> CB1OF_R {
        CB1OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear buffer 2 overflow flag Writing 1 clears the buffer 2 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb2of(&self) -> CB2OF_R {
        CB2OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear buffer 3 overflow flag Writing 1 clears the buffer 3 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb3of(&self) -> CB3OF_R {
        CB3OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear AHB master error flag Writing 1 clears the AHB master error flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn camef(&self) -> CAMEF_R {
        CAMEF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("cb0of", &self.cb0of())
            .field("cb1of", &self.cb1of())
            .field("cb2of", &self.cb2of())
            .field("cb3of", &self.cb3of())
            .field("camef", &self.camef())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear buffer 0 overflow flag Writing 1 clears the buffer 0 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb0of(&mut self) -> CB0OF_W<FCRrs> {
        CB0OF_W::new(self, 0)
    }
    ///Bit 1 - Clear buffer 1 overflow flag Writing 1 clears the buffer 1 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb1of(&mut self) -> CB1OF_W<FCRrs> {
        CB1OF_W::new(self, 1)
    }
    ///Bit 2 - Clear buffer 2 overflow flag Writing 1 clears the buffer 2 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb2of(&mut self) -> CB2OF_W<FCRrs> {
        CB2OF_W::new(self, 2)
    }
    ///Bit 3 - Clear buffer 3 overflow flag Writing 1 clears the buffer 3 overflow flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn cb3of(&mut self) -> CB3OF_W<FCRrs> {
        CB3OF_W::new(self, 3)
    }
    ///Bit 4 - Clear AHB master error flag Writing 1 clears the AHB master error flag in the GFXMMU_SR register.
    #[inline(always)]
    pub fn camef(&mut self) -> CAMEF_W<FCRrs> {
        CAMEF_W::new(self, 4)
    }
}
/**GFXMMU flag clear register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GFXMMU:FCR)*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`read()` method returns [`fcr::R`](R) reader structure
impl crate::Readable for FCRrs {}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
