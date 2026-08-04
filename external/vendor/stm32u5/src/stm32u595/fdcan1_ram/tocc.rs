///Register `TOCC` reader
pub type R = crate::R<TOCCrs>;
///Register `TOCC` writer
pub type W = crate::W<TOCCrs>;
///Field `ETOC` reader - Enable Timeout Counter
pub type ETOC_R = crate::BitReader;
///Field `ETOC` writer - Enable Timeout Counter
pub type ETOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOS` reader - Timeout Select
pub type TOS_R = crate::FieldReader;
///Field `TOS` writer - Timeout Select
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOP` reader - Timeout Period
pub type TOP_R = crate::FieldReader<u16>;
///Field `TOP` writer - Timeout Period
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Enable Timeout Counter
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Timeout Select
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 16:31 - Timeout Period
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOCC")
            .field("etoc", &self.etoc())
            .field("tos", &self.tos())
            .field("top", &self.top())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Timeout Counter
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W<TOCCrs> {
        ETOC_W::new(self, 0)
    }
    ///Bits 1:2 - Timeout Select
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W<TOCCrs> {
        TOS_W::new(self, 1)
    }
    ///Bits 16:31 - Timeout Period
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W<TOCCrs> {
        TOP_W::new(self, 16)
    }
}
/**FDCAN Timeout Counter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:TOCC)*/
pub struct TOCCrs;
impl crate::RegisterSpec for TOCCrs {
    type Ux = u32;
}
///`read()` method returns [`tocc::R`](R) reader structure
impl crate::Readable for TOCCrs {}
///`write(|w| ..)` method takes [`tocc::W`](W) writer structure
impl crate::Writable for TOCCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TOCC to value 0xffff_0000
impl crate::Resettable for TOCCrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
