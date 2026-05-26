///Register `ILE` reader
pub type R = crate::R<ILErs>;
///Register `ILE` writer
pub type W = crate::W<ILErs>;
///Field `EINT0` reader - Enable Interrupt Line 0
pub type EINT0_R = crate::BitReader;
///Field `EINT0` writer - Enable Interrupt Line 0
pub type EINT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EINT1` reader - Enable Interrupt Line 1
pub type EINT1_R = crate::BitReader;
///Field `EINT1` writer - Enable Interrupt Line 1
pub type EINT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Interrupt Line 0
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Interrupt Line 1
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILE")
            .field("eint0", &self.eint0())
            .field("eint1", &self.eint1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Interrupt Line 0
    #[inline(always)]
    pub fn eint0(&mut self) -> EINT0_W<ILErs> {
        EINT0_W::new(self, 0)
    }
    ///Bit 1 - Enable Interrupt Line 1
    #[inline(always)]
    pub fn eint1(&mut self) -> EINT1_W<ILErs> {
        EINT1_W::new(self, 1)
    }
}
/**FDCAN Interrupt Line Enable Register

You can [`read`](crate::Reg::read) this register and get [`ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#FDCAN1_RAM:ILE)*/
pub struct ILErs;
impl crate::RegisterSpec for ILErs {
    type Ux = u32;
}
///`read()` method returns [`ile::R`](R) reader structure
impl crate::Readable for ILErs {}
///`write(|w| ..)` method takes [`ile::W`](W) writer structure
impl crate::Writable for ILErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ILE to value 0
impl crate::Resettable for ILErs {}
