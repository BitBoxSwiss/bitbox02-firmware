///Register `PARAM` reader
pub type R = crate::R<PARAMrs>;
///Register `PARAM` writer
pub type W = crate::W<PARAMrs>;
///Field `P` reader - Input parameter P
pub type P_R = crate::FieldReader;
///Field `P` writer - Input parameter P
pub type P_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `Q` reader - Input parameter Q
pub type Q_R = crate::FieldReader;
///Field `Q` writer - Input parameter Q
pub type Q_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `R` reader - Input parameter R
pub type R_R = crate::FieldReader;
///Field `R` writer - Input parameter R
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FUNC` reader - Function
pub type FUNC_R = crate::FieldReader;
///Field `FUNC` writer - Function
pub type FUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `START` reader - Enable execution
pub type START_R = crate::BitReader;
///Field `START` writer - Enable execution
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Input parameter P
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Input parameter Q
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Input parameter R
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - Function
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    ///Bit 31 - Enable execution
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("start", &self.start())
            .field("func", &self.func())
            .field("r", &self.r())
            .field("q", &self.q())
            .field("p", &self.p())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Input parameter P
    #[inline(always)]
    pub fn p(&mut self) -> P_W<PARAMrs> {
        P_W::new(self, 0)
    }
    ///Bits 8:15 - Input parameter Q
    #[inline(always)]
    pub fn q(&mut self) -> Q_W<PARAMrs> {
        Q_W::new(self, 8)
    }
    ///Bits 16:23 - Input parameter R
    #[inline(always)]
    pub fn r(&mut self) -> R_W<PARAMrs> {
        R_W::new(self, 16)
    }
    ///Bits 24:30 - Function
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<PARAMrs> {
        FUNC_W::new(self, 24)
    }
    ///Bit 31 - Enable execution
    #[inline(always)]
    pub fn start(&mut self) -> START_W<PARAMrs> {
        START_W::new(self, 31)
    }
}
/**FMAC Parameter register

You can [`read`](crate::Reg::read) this register and get [`param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FMAC:PARAM)*/
pub struct PARAMrs;
impl crate::RegisterSpec for PARAMrs {
    type Ux = u32;
}
///`read()` method returns [`param::R`](R) reader structure
impl crate::Readable for PARAMrs {}
///`write(|w| ..)` method takes [`param::W`](W) writer structure
impl crate::Writable for PARAMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PARAM to value 0
impl crate::Resettable for PARAMrs {}
