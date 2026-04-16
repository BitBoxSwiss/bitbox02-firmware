///Register `CCCSR` reader
pub type R = crate::R<CCCSRrs>;
///Register `CCCSR` writer
pub type W = crate::W<CCCSRrs>;
///Field `EN1` reader - EN1
pub type EN1_R = crate::BitReader;
///Field `EN1` writer - EN1
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS1` reader - CS1
pub type CS1_R = crate::BitReader;
///Field `CS1` writer - CS1
pub type CS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN2` reader - EN2
pub type EN2_R = crate::BitReader;
///Field `EN2` writer - EN2
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS2` reader - CS2
pub type CS2_R = crate::BitReader;
///Field `CS2` writer - CS2
pub type CS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN3` reader - EN3
pub type EN3_R = crate::BitReader;
///Field `EN3` writer - EN3
pub type EN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS3` reader - CS3
pub type CS3_R = crate::BitReader;
///Field `CS3` writer - CS3
pub type CS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDY1` reader - RDY1
pub type RDY1_R = crate::BitReader;
///Field `RDY2` reader - RDY2
pub type RDY2_R = crate::BitReader;
///Field `RDY3` reader - RDY3
pub type RDY3_R = crate::BitReader;
impl R {
    ///Bit 0 - EN1
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CS1
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EN2
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CS2
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EN3
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CS3
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - RDY1
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RDY2
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RDY3
    #[inline(always)]
    pub fn rdy3(&self) -> RDY3_R {
        RDY3_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCSR")
            .field("en1", &self.en1())
            .field("cs1", &self.cs1())
            .field("en2", &self.en2())
            .field("cs2", &self.cs2())
            .field("en3", &self.en3())
            .field("cs3", &self.cs3())
            .field("rdy1", &self.rdy1())
            .field("rdy2", &self.rdy2())
            .field("rdy3", &self.rdy3())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN1
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<CCCSRrs> {
        EN1_W::new(self, 0)
    }
    ///Bit 1 - CS1
    #[inline(always)]
    pub fn cs1(&mut self) -> CS1_W<CCCSRrs> {
        CS1_W::new(self, 1)
    }
    ///Bit 2 - EN2
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<CCCSRrs> {
        EN2_W::new(self, 2)
    }
    ///Bit 3 - CS2
    #[inline(always)]
    pub fn cs2(&mut self) -> CS2_W<CCCSRrs> {
        CS2_W::new(self, 3)
    }
    ///Bit 4 - EN3
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W<CCCSRrs> {
        EN3_W::new(self, 4)
    }
    ///Bit 5 - CS3
    #[inline(always)]
    pub fn cs3(&mut self) -> CS3_W<CCCSRrs> {
        CS3_W::new(self, 5)
    }
}
/**compensation cell control/status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#SYSCFG:CCCSR)*/
pub struct CCCSRrs;
impl crate::RegisterSpec for CCCSRrs {
    type Ux = u32;
}
///`read()` method returns [`cccsr::R`](R) reader structure
impl crate::Readable for CCCSRrs {}
///`write(|w| ..)` method takes [`cccsr::W`](W) writer structure
impl crate::Writable for CCCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCSR to value 0x0a
impl crate::Resettable for CCCSRrs {
    const RESET_VALUE: u32 = 0x0a;
}
