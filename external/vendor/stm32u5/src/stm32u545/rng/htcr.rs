///Register `HTCR` reader
pub type R = crate::R<HTCRrs>;
///Register `HTCR` writer
pub type W = crate::W<HTCRrs>;
/**health test configuration

Value on reset: 25204*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum HTCFG {
    ///43636: Recommended value for RNG certification (0x0000_AA74)
    Recommended = 43636,
    ///391711420: Magic number to be written before any write (0x1759_0ABC)
    Magic = 391711420,
}
impl From<HTCFG> for u32 {
    #[inline(always)]
    fn from(variant: HTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HTCFG {
    type Ux = u32;
}
impl crate::IsEnum for HTCFG {}
///Field `HTCFG` reader - health test configuration
pub type HTCFG_R = crate::FieldReader<HTCFG>;
impl HTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<HTCFG> {
        match self.bits {
            43636 => Some(HTCFG::Recommended),
            391711420 => Some(HTCFG::Magic),
            _ => None,
        }
    }
    ///Recommended value for RNG certification (0x0000_AA74)
    #[inline(always)]
    pub fn is_recommended(&self) -> bool {
        *self == HTCFG::Recommended
    }
    ///Magic number to be written before any write (0x1759_0ABC)
    #[inline(always)]
    pub fn is_magic(&self) -> bool {
        *self == HTCFG::Magic
    }
}
///Field `HTCFG` writer - health test configuration
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, HTCFG>;
impl<'a, REG> HTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Recommended value for RNG certification (0x0000_AA74)
    #[inline(always)]
    pub fn recommended(self) -> &'a mut crate::W<REG> {
        self.variant(HTCFG::Recommended)
    }
    ///Magic number to be written before any write (0x1759_0ABC)
    #[inline(always)]
    pub fn magic(self) -> &'a mut crate::W<REG> {
        self.variant(HTCFG::Magic)
    }
}
impl R {
    ///Bits 0:31 - health test configuration
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTCR")
            .field("htcfg", &self.htcfg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - health test configuration
    #[inline(always)]
    pub fn htcfg(&mut self) -> HTCFG_W<HTCRrs> {
        HTCFG_W::new(self, 0)
    }
}
/**health test control register

You can [`read`](crate::Reg::read) this register and get [`htcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RNG:HTCR)*/
pub struct HTCRrs;
impl crate::RegisterSpec for HTCRrs {
    type Ux = u32;
}
///`read()` method returns [`htcr::R`](R) reader structure
impl crate::Readable for HTCRrs {}
///`write(|w| ..)` method takes [`htcr::W`](W) writer structure
impl crate::Writable for HTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTCR to value 0x6274
impl crate::Resettable for HTCRrs {
    const RESET_VALUE: u32 = 0x6274;
}
