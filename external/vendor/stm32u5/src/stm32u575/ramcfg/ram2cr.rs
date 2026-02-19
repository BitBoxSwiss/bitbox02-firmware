///Register `RAM2CR` reader
pub type R = crate::R<RAM2CRrs>;
///Register `RAM2CR` writer
pub type W = crate::W<RAM2CRrs>;
///Field `ECCE` reader - ECCE
pub type ECCE_R = crate::BitReader;
///Field `ECCE` writer - ECCE
pub type ECCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALE` reader - ALE
pub type ALE_R = crate::BitReader;
///Field `ALE` writer - ALE
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMER` reader - SRAMER
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAMER
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WSC` reader - WSC
pub type WSC_R = crate::FieldReader;
///Field `WSC` writer - WSC
pub type WSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - ECCE
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ALE
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SRAMER
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:18 - WSC
    #[inline(always)]
    pub fn wsc(&self) -> WSC_R {
        WSC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM2CR")
            .field("ecce", &self.ecce())
            .field("ale", &self.ale())
            .field("sramer", &self.sramer())
            .field("wsc", &self.wsc())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECCE
    #[inline(always)]
    pub fn ecce(&mut self) -> ECCE_W<RAM2CRrs> {
        ECCE_W::new(self, 0)
    }
    ///Bit 4 - ALE
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W<RAM2CRrs> {
        ALE_W::new(self, 4)
    }
    ///Bit 8 - SRAMER
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<RAM2CRrs> {
        SRAMER_W::new(self, 8)
    }
    ///Bits 16:18 - WSC
    #[inline(always)]
    pub fn wsc(&mut self) -> WSC_W<RAM2CRrs> {
        WSC_W::new(self, 16)
    }
}
/**RAMCFG SRAM x control register

You can [`read`](crate::Reg::read) this register and get [`ram2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RAMCFG:RAM2CR)*/
pub struct RAM2CRrs;
impl crate::RegisterSpec for RAM2CRrs {
    type Ux = u32;
}
///`read()` method returns [`ram2cr::R`](R) reader structure
impl crate::Readable for RAM2CRrs {}
///`write(|w| ..)` method takes [`ram2cr::W`](W) writer structure
impl crate::Writable for RAM2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RAM2CR to value 0
impl crate::Resettable for RAM2CRrs {}
