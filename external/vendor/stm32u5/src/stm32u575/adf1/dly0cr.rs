///Register `DLY0CR` reader
pub type R = crate::R<DLY0CRrs>;
///Register `DLY0CR` writer
pub type W = crate::W<DLY0CRrs>;
///Field `SKPDLY` reader - Delay to apply to a bitstream
pub type SKPDLY_R = crate::FieldReader;
///Field `SKPDLY` writer - Delay to apply to a bitstream
pub type SKPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SKPBF` reader - Skip busy flag
pub type SKPBF_R = crate::BitReader;
///Field `SKPBF` writer - Skip busy flag
pub type SKPBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - Delay to apply to a bitstream
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 31 - Skip busy flag
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY0CR")
            .field("skpbf", &self.skpbf())
            .field("skpdly", &self.skpdly())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Delay to apply to a bitstream
    #[inline(always)]
    pub fn skpdly(&mut self) -> SKPDLY_W<DLY0CRrs> {
        SKPDLY_W::new(self, 0)
    }
    ///Bit 31 - Skip busy flag
    #[inline(always)]
    pub fn skpbf(&mut self) -> SKPBF_W<DLY0CRrs> {
        SKPBF_W::new(self, 31)
    }
}
/**ADF delay control register 0

You can [`read`](crate::Reg::read) this register and get [`dly0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADF1:DLY0CR)*/
pub struct DLY0CRrs;
impl crate::RegisterSpec for DLY0CRrs {
    type Ux = u32;
}
///`read()` method returns [`dly0cr::R`](R) reader structure
impl crate::Readable for DLY0CRrs {}
///`write(|w| ..)` method takes [`dly0cr::W`](W) writer structure
impl crate::Writable for DLY0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLY0CR to value 0
impl crate::Resettable for DLY0CRrs {}
