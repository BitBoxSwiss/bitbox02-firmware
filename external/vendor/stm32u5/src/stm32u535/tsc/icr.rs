///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `EOAIC` reader - End of acquisition interrupt clear
pub type EOAIC_R = crate::BitReader;
///Field `EOAIC` writer - End of acquisition interrupt clear
pub type EOAIC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCEIC` reader - Max count error interrupt clear
pub type MCEIC_R = crate::BitReader;
///Field `MCEIC` writer - Max count error interrupt clear
pub type MCEIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - End of acquisition interrupt clear
    #[inline(always)]
    pub fn eoaic(&self) -> EOAIC_R {
        EOAIC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error interrupt clear
    #[inline(always)]
    pub fn mceic(&self) -> MCEIC_R {
        MCEIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("mceic", &self.mceic())
            .field("eoaic", &self.eoaic())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of acquisition interrupt clear
    #[inline(always)]
    pub fn eoaic(&mut self) -> EOAIC_W<ICRrs> {
        EOAIC_W::new(self, 0)
    }
    ///Bit 1 - Max count error interrupt clear
    #[inline(always)]
    pub fn mceic(&mut self) -> MCEIC_W<ICRrs> {
        MCEIC_W::new(self, 1)
    }
}
/**interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#TSC:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
