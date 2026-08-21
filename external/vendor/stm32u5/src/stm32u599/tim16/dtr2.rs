///Register `DTR2` reader
pub type R = crate::R<DTR2rs>;
///Register `DTR2` writer
pub type W = crate::W<DTR2rs>;
///Field `DTGF` reader - Deadtime asymmetric enable
pub type DTGF_R = crate::FieldReader;
///Field `DTGF` writer - Deadtime asymmetric enable
pub type DTGF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTAE` reader - Deadtime asymmetric enable
pub type DTAE_R = crate::BitReader;
///Field `DTAE` writer - Deadtime asymmetric enable
pub type DTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTPE` reader - Deadtime preload enable
pub type DTPE_R = crate::BitReader;
///Field `DTPE` writer - Deadtime preload enable
pub type DTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Deadtime asymmetric enable
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - Deadtime asymmetric enable
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Deadtime preload enable
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTR2")
            .field("dtpe", &self.dtpe())
            .field("dtae", &self.dtae())
            .field("dtgf", &self.dtgf())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Deadtime asymmetric enable
    #[inline(always)]
    pub fn dtgf(&mut self) -> DTGF_W<DTR2rs> {
        DTGF_W::new(self, 0)
    }
    ///Bit 16 - Deadtime asymmetric enable
    #[inline(always)]
    pub fn dtae(&mut self) -> DTAE_W<DTR2rs> {
        DTAE_W::new(self, 16)
    }
    ///Bit 17 - Deadtime preload enable
    #[inline(always)]
    pub fn dtpe(&mut self) -> DTPE_W<DTR2rs> {
        DTPE_W::new(self, 17)
    }
}
/**timer deadtime register 2

You can [`read`](crate::Reg::read) this register and get [`dtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#TIM16:DTR2)*/
pub struct DTR2rs;
impl crate::RegisterSpec for DTR2rs {
    type Ux = u32;
}
///`read()` method returns [`dtr2::R`](R) reader structure
impl crate::Readable for DTR2rs {}
///`write(|w| ..)` method takes [`dtr2::W`](W) writer structure
impl crate::Writable for DTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTR2 to value 0
impl crate::Resettable for DTR2rs {}
