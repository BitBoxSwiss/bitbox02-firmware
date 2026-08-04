///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `BYTE0` reader - Data byte 0
pub type BYTE0_R = crate::FieldReader;
///Field `BYTE0` writer - Data byte 0
pub type BYTE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BYTE1` reader - Data byte 1
pub type BYTE1_R = crate::FieldReader;
///Field `BYTE1` writer - Data byte 1
pub type BYTE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BYTE2` reader - Data byte 2
pub type BYTE2_R = crate::FieldReader;
///Field `BYTE2` writer - Data byte 2
pub type BYTE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `BYTE3` reader - Data byte 3
pub type BYTE3_R = crate::FieldReader;
///Field `BYTE3` writer - Data byte 3
pub type BYTE3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Data byte 0
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data byte 1
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data byte 2
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data byte 3
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("byte0", &self.byte0())
            .field("byte1", &self.byte1())
            .field("byte2", &self.byte2())
            .field("byte3", &self.byte3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data byte 0
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W<DRrs> {
        BYTE0_W::new(self, 0)
    }
    ///Bits 8:15 - Data byte 1
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W<DRrs> {
        BYTE1_W::new(self, 8)
    }
    ///Bits 16:23 - Data byte 2
    #[inline(always)]
    pub fn byte2(&mut self) -> BYTE2_W<DRrs> {
        BYTE2_W::new(self, 16)
    }
    ///Bits 24:31 - Data byte 3
    #[inline(always)]
    pub fn byte3(&mut self) -> BYTE3_W<DRrs> {
        BYTE3_W::new(self, 24)
    }
}
/**PSSI data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PSSI:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR to value 0xc000_0000
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
