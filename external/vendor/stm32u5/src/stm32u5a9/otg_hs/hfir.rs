///Register `HFIR` reader
pub type R = crate::R<HFIRrs>;
///Register `HFIR` writer
pub type W = crate::W<HFIRrs>;
///Field `FRIVL` reader - FRIVL
pub type FRIVL_R = crate::FieldReader<u16>;
///Field `FRIVL` writer - FRIVL
pub type FRIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RLDCTRL` reader - RLDCTRL
pub type RLDCTRL_R = crate::BitReader;
///Field `RLDCTRL` writer - RLDCTRL
pub type RLDCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - FRIVL
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - RLDCTRL
    #[inline(always)]
    pub fn rldctrl(&self) -> RLDCTRL_R {
        RLDCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFIR")
            .field("frivl", &self.frivl())
            .field("rldctrl", &self.rldctrl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - FRIVL
    #[inline(always)]
    pub fn frivl(&mut self) -> FRIVL_W<HFIRrs> {
        FRIVL_W::new(self, 0)
    }
    ///Bit 16 - RLDCTRL
    #[inline(always)]
    pub fn rldctrl(&mut self) -> RLDCTRL_W<HFIRrs> {
        RLDCTRL_W::new(self, 16)
    }
}
/**This register stores the frame interval information for the current speed to which the OTG controller has enumerated.

You can [`read`](crate::Reg::read) this register and get [`hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTG_HS:HFIR)*/
pub struct HFIRrs;
impl crate::RegisterSpec for HFIRrs {
    type Ux = u32;
}
///`read()` method returns [`hfir::R`](R) reader structure
impl crate::Readable for HFIRrs {}
///`write(|w| ..)` method takes [`hfir::W`](W) writer structure
impl crate::Writable for HFIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HFIR to value 0xea60
impl crate::Resettable for HFIRrs {
    const RESET_VALUE: u32 = 0xea60;
}
